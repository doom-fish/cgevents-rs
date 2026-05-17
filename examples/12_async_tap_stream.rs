//! Example: async `CGEvent` tap stream
//!
//! Subscribes to all keyboard events via an async stream and prints the
//! first event (or times out after 2 s).
//!
//! Requires Accessibility permission. The example exits 0 when permission
//! is not granted (graceful skip for CI / headless machines).

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires --features async");
    std::process::exit(0);
}

#[cfg(feature = "async")]
fn main() {
    use cgevents::async_api::CGEventTapStream;
    use cgevents::tap::EventTap;
    use cgevents::{CGEventType, TapLocation};

    // Gracefully skip on machines that have not granted Accessibility access.
    if !EventTap::preflight_listen_access() {
        println!("Accessibility permission not granted — skipping tap stream example.");
        std::process::exit(0);
    }

    let keyboard_mask = CGEventType::KeyDown.mask_bit() | CGEventType::KeyUp.mask_bit();

    let stream = match CGEventTapStream::subscribe(TapLocation::Session, keyboard_mask, 64) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not create tap: {e:?}");
            std::process::exit(1);
        }
    };

    println!("Tap installed. Waiting up to 2 s for a keyboard event…");

    let got = pollster::block_on(async {
        // Race stream.next() against a 2-second timeout.
        let timeout = std::time::Duration::from_secs(2);
        let start = std::time::Instant::now();
        loop {
            if let Some(ev) = stream.try_next() {
                return Some(ev);
            }
            if start.elapsed() >= timeout {
                return None;
            }
            // Yield to let the async runtime poll the stream.
            std::future::poll_fn(|cx| {
                cx.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
    });

    match got {
        Some(ev) => {
            println!(
                "Got event: type={:?} ({}) keycode={} flags={:?}",
                ev.event_type, ev.event_type_raw, ev.keycode, ev.flags
            );
        }
        None => println!("No keyboard event received within the timeout — that's fine."),
    }

    // The stream (and its run-loop thread) are dropped here.
    println!("Stream dropped cleanly.");
}
