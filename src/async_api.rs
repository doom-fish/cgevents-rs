//! Async event-stream API for `CGEvent` taps.
//!
//! This module (enabled by the `async` Cargo feature) wraps the
//! `CGEventTapCreate` callback mechanism as an executor-agnostic
//! [`BoundedAsyncStream`](doom_fish_utils::stream::BoundedAsyncStream).
//!
//! # Overview
//!
//! [`CGEventTapStream`] installs a **listen-only**, passive `CGEvent` tap on a
//! dedicated run-loop thread and delivers every intercepted event as an
//! owned [`CGEventItem`] snapshot.  Because event taps that fall behind are
//! silently disabled by macOS, the stream uses the default **lossy** policy:
//! when the internal ring buffer is full, the **oldest** buffered item is
//! dropped to make room for the incoming one.
//!
//! # Permissions
//!
//! `CGEvent` taps require the Accessibility permission.  The constructor
//! returns [`CGError::TapCreateFailed`] when access is denied.  Check
//! [`crate::tap::EventTap::preflight_listen_access`] before subscribing.
//!
//! # Example
//!
//! ```no_run
//! use cgevents::async_api::CGEventTapStream;
//! use cgevents::{TapLocation, CG_EVENT_MASK_FOR_ALL_EVENTS};
//!
//! # async fn run() -> Result<(), cgevents::CGError> {
//! let stream = CGEventTapStream::subscribe(
//!     TapLocation::Session,
//!     CG_EVENT_MASK_FOR_ALL_EVENTS,
//!     64,
//! )?;
//!
//! // Receive events asynchronously (works with any executor).
//! while let Some(event) = stream.next().await {
//!     println!("{:?}  @ ({:.0}, {:.0})", event.event_type, event.location.x, event.location.y);
//! }
//! # Ok(())
//! # }
//! ```

use crate::cg_event_flags::CGEventFlags;
use crate::cg_event_tap_location::TapLocation;
use crate::cg_event_timestamp::CGEventTimestamp;
use crate::cg_event_type::CGEventType;
use crate::error::CGError;
use crate::event::Point;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream};
use std::ffi::c_void;

// ── FFI bindings ─────────────────────────────────────────────────────────────

mod ffi {
    use std::ffi::c_void;

    /// `C` callback invoked on every tapped event.
    /// Arguments: `event_type`, `location_x`, `location_y`, `flags`, `timestamp`, `keycode`, `ctx`.
    pub type CGEventsStreamCallback = unsafe extern "C" fn(
        event_type: u32,
        location_x: f64,
        location_y: f64,
        flags: u64,
        timestamp: u64,
        keycode: u16,
        ctx: *mut c_void,
    );

    unsafe extern "C" {
        /// Install a listen-only `CGEvent` tap on a dedicated run-loop thread.
        ///
        /// Returns an opaque handle or null on failure.
        pub fn cgevents_tap_stream_subscribe(
            location: u32,
            events_of_interest: u64,
            callback: CGEventsStreamCallback,
            ctx: *mut c_void,
        ) -> *mut c_void;

        /// Disable the tap, stop the run-loop thread, and release the bridge.
        pub fn cgevents_tap_stream_unsubscribe(handle: *mut c_void);
    }
}

// ── Public event item ─────────────────────────────────────────────────────────

/// An owned snapshot of one intercepted `CGEvent`.
///
/// Unlike [`crate::tap::TappedEvent`] (which is a borrowed view valid only
/// for the duration of the tap callback), `CGEventItem` is fully owned and
/// can be sent across thread boundaries.
#[derive(Debug, Clone, PartialEq)]
pub struct CGEventItem {
    /// Raw event-type value as reported by CoreGraphics.
    pub event_type_raw: u32,
    /// Typed event kind, or `None` for unrecognised system events.
    pub event_type: Option<CGEventType>,
    /// Screen location at the time of the event (flipped coordinates,
    /// origin at top-left, units = points).
    pub location: Point,
    /// Active modifier flags.
    pub flags: CGEventFlags,
    /// Mach absolute time at which the event occurred.
    pub timestamp: CGEventTimestamp,
    /// Virtual key code for keyboard events; `0` for pointer/scroll events.
    pub keycode: u16,
}

// ── RAII subscription handle ─────────────────────────────────────────────────

/// Drops the Swift bridge and frees the stream sender.
///
/// Calling the Swift unsubscribe first stops the run-loop thread; only after
/// that is the sender pointer freed, ensuring no concurrent callback can
/// observe a freed pointer.
struct TapSubscriptionHandle {
    swift_handle: *mut c_void,
    sender_ptr: *mut AsyncStreamSender<CGEventItem>,
}

// Safety: the only operations performed on the raw pointers are serialised
// through Swift's run-loop thread (which is stopped before the sender is
// freed) and the Drop impl below.
unsafe impl Send for TapSubscriptionHandle {}
unsafe impl Sync for TapSubscriptionHandle {}

impl Drop for TapSubscriptionHandle {
    fn drop(&mut self) {
        if !self.swift_handle.is_null() {
            // Disables the tap, stops the run-loop thread, and blocks until
            // the thread has fully exited (via `stopAndJoin()` on the Swift
            // side).  After this returns, no more `tap_stream_callback`
            // invocations can occur and it is safe to free the sender pointer.
            unsafe { ffi::cgevents_tap_stream_unsubscribe(self.swift_handle) };
            self.swift_handle = std::ptr::null_mut();
        }
        if !self.sender_ptr.is_null() {
            // Safe: the run-loop thread has fully exited; no live references
            // to sender_ptr remain.
            unsafe { drop(Box::from_raw(self.sender_ptr)) };
            self.sender_ptr = std::ptr::null_mut();
        }
    }
}

// ── CGEvent tap callback ──────────────────────────────────────────────────────

/// Invoked on the tap's run-loop thread for every matching event.
///
/// Reconstructs the `AsyncStreamSender` from `ctx` and pushes a
/// [`CGEventItem`] snapshot.  The push is lossy: if the ring buffer is
/// full, the oldest item is silently dropped.
extern "C" fn tap_stream_callback(
    event_type: u32,
    location_x: f64,
    location_y: f64,
    flags: u64,
    timestamp: u64,
    keycode: u16,
    ctx: *mut c_void,
) {
    if ctx.is_null() {
        return;
    }
    // Safety: `ctx` is a valid `AsyncStreamSender<CGEventItem>` for the
    // lifetime of the subscription (guaranteed by `TapSubscriptionHandle`).
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<CGEventItem>>() };
    sender.push(CGEventItem {
        event_type_raw: event_type,
        event_type: CGEventType::from_raw(event_type),
        location: Point::new(location_x, location_y),
        flags: CGEventFlags::from_bits_truncate(flags),
        timestamp: CGEventTimestamp(timestamp),
        keycode,
    });
}

// ── Public stream type ────────────────────────────────────────────────────────

/// An async stream of [`CGEventItem`]s delivered by a `CGEvent` tap.
///
/// Owns a dedicated run-loop thread that drives the underlying
/// `CGEventTap`.  Dropping the stream disables the tap, stops the thread,
/// and closes the channel.
///
/// ## Lossy semantics
///
/// The internal ring buffer uses the default **drop-oldest** policy.  A
/// slow consumer will miss events rather than stalling the tap thread —
/// matching macOS's own behaviour (stalling taps are silently disabled).
pub struct CGEventTapStream {
    inner: BoundedAsyncStream<CGEventItem>,
    _handle: TapSubscriptionHandle,
}

impl CGEventTapStream {
    /// Subscribe to a `CGEvent` tap and return an async stream.
    ///
    /// Installs a listen-only tap at `location` observing every event type
    /// whose bit is set in `events_mask`.  Use
    /// [`CG_EVENT_MASK_FOR_ALL_EVENTS`](crate::cg_event_type::CG_EVENT_MASK_FOR_ALL_EVENTS)
    /// to observe everything.
    ///
    /// # Arguments
    ///
    /// * `location`    — session / HID / annotated-session tap point.
    /// * `events_mask` — bitmask of [`CGEventType::mask_bit()`] values.
    /// * `capacity`    — ring-buffer depth; 64 is a reasonable default.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::TapCreateFailed`] when macOS refuses to create the
    /// tap — most commonly because the Accessibility permission has not been
    /// granted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use cgevents::async_api::CGEventTapStream;
    /// use cgevents::{TapLocation, CGEventType};
    ///
    /// # async fn run() -> Result<(), cgevents::CGError> {
    /// let mask = CGEventType::KeyDown.mask_bit() | CGEventType::KeyUp.mask_bit();
    /// let stream = CGEventTapStream::subscribe(TapLocation::Session, mask, 64)?;
    /// while let Some(ev) = stream.next().await {
    ///     if let Some(t) = ev.event_type {
    ///         println!("key event: {t:?}  keycode={}", ev.keycode);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribe(
        location: TapLocation,
        events_mask: u64,
        capacity: usize,
    ) -> Result<Self, CGError> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));

        // Safety: `sender_ptr` is a valid, heap-allocated
        // `AsyncStreamSender<CGEventItem>`; it is freed in
        // `TapSubscriptionHandle::drop` after the run-loop thread has stopped.
        let swift_handle = unsafe {
            ffi::cgevents_tap_stream_subscribe(
                location.raw(),
                events_mask,
                tap_stream_callback,
                sender_ptr.cast(),
            )
        };

        if swift_handle.is_null() {
            // No callbacks will fire; reclaim the sender immediately.
            unsafe { drop(Box::from_raw(sender_ptr)) };
            return Err(CGError::TapCreateFailed);
        }

        Ok(Self {
            inner: stream,
            _handle: TapSubscriptionHandle {
                swift_handle,
                sender_ptr,
            },
        })
    }

    /// Await the next event from the tap.
    ///
    /// Returns `None` when the stream has been closed (i.e., the
    /// `CGEventTapStream` has been dropped and the buffer is empty).
    #[must_use]
    pub const fn next(&self) -> doom_fish_utils::stream::NextItem<'_, CGEventItem> {
        self.inner.next()
    }

    /// Non-blocking poll: returns `Some(event)` if one is buffered, else `None`.
    #[must_use]
    pub fn try_next(&self) -> Option<CGEventItem> {
        self.inner.try_next()
    }

    /// Number of events currently buffered (≤ capacity).
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// `true` after the stream has been closed and drained.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
