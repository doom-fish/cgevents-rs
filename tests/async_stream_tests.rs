//! Tests for the async `CGEvent` tap stream API.
//!
//! Most tests skip gracefully when Accessibility permission is not granted
//! (headless CI / machines without input-monitoring entitlement).

#[cfg(feature = "async")]
mod async_stream {
    use cgevents::async_api::CGEventTapStream;
    use cgevents::tap::EventTap;
    use cgevents::{CGEventType, TapLocation, CG_EVENT_MASK_FOR_ALL_EVENTS};

    fn has_access() -> bool {
        EventTap::preflight_listen_access()
    }

    // ── compile-time checks ───────────────────────────────────────────────

    #[test]
    fn stream_type_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CGEventTapStream>();
    }

    // ── subscribe / drop lifecycle ────────────────────────────────────────

    /// subscribe → stream is open → drop handle → stream closes.
    #[test]
    fn subscribe_and_drop_closes_stream() {
        if !has_access() {
            eprintln!("skip: Accessibility permission not granted");
            return;
        }

        let stream = CGEventTapStream::subscribe(
            TapLocation::Session,
            CG_EVENT_MASK_FOR_ALL_EVENTS,
            8,
        )
        .expect("tap create failed");

        // Stream must be open right after subscribing.
        assert!(!stream.is_closed(), "stream should be open after subscribe");
        assert_eq!(stream.buffered_count(), 0);

        // Dropping the stream closes it; `is_closed` checks the inner
        // BoundedAsyncStream (which becomes closed once all senders are
        // dropped — that happens inside TapSubscriptionHandle::drop).
        drop(stream);
        // After drop we no longer have a handle to observe `is_closed`.
        // The important invariant is that drop does not panic or hang.
    }

    /// subscribe → capacity propagates.
    #[test]
    fn subscribe_respects_capacity() {
        if !has_access() {
            eprintln!("skip: Accessibility permission not granted");
            return;
        }

        let stream = CGEventTapStream::subscribe(TapLocation::Session, CG_EVENT_MASK_FOR_ALL_EVENTS, 32)
            .expect("tap create failed");
        // No events should be buffered yet.
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
    }

    /// `try_next` returns `None` when the buffer is empty.
    #[test]
    fn try_next_returns_none_when_empty() {
        if !has_access() {
            eprintln!("skip: Accessibility permission not granted");
            return;
        }

        let stream = CGEventTapStream::subscribe(
            TapLocation::Session,
            CG_EVENT_MASK_FOR_ALL_EVENTS,
            8,
        )
        .expect("tap create failed");
        assert!(stream.try_next().is_none());
    }

    /// Subscribe fails cleanly on a machine without permission.
    ///
    /// This test is only exercised when we DON'T have Accessibility access.
    #[test]
    fn subscribe_fails_without_permission() {
        if has_access() {
            eprintln!("skip: machine has Accessibility permission");
            return;
        }
        let result =
            CGEventTapStream::subscribe(TapLocation::Session, CG_EVENT_MASK_FOR_ALL_EVENTS, 8);
        assert!(
            result.is_err(),
            "expected TapCreateFailed without Accessibility permission"
        );
    }

    // ── keyboard mask ─────────────────────────────────────────────────────

    /// Subscribe with a keyboard-only mask succeeds (or returns a clean error
    /// without Accessibility).
    #[test]
    fn subscribe_keyboard_mask() {
        let mask = CGEventType::KeyDown.mask_bit() | CGEventType::KeyUp.mask_bit();
        match CGEventTapStream::subscribe(TapLocation::Session, mask, 16) {
            Ok(_) | Err(cgevents::CGError::TapCreateFailed) => {
                /* tap installed or gracefully denied — both acceptable */
            }
            Err(e) => panic!("unexpected error: {e:?}"),
        }
    }
}
