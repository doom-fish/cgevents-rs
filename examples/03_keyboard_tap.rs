//! Tap (intercept) every keyboard event globally and print it. Demonstrates
//! that the API works even when this binary is not focused.
//!
//! Run: `cargo run --example 03_keyboard_tap`
//!
//! REQUIRES Accessibility permission. The first run will fail with
//! `CGError::TapCreateFailed` — open
//! System Settings → Privacy & Security → Accessibility, add the binary
//! at `target/debug/examples/03_keyboard_tap`, and re-run.

use cgevents::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = Arc::new(AtomicUsize::new(0));
    let count_cb = count;

    let tap = match EventTap::keyboard(move |event| {
        let n = count_cb.fetch_add(1, Ordering::SeqCst) + 1;
        println!(
            "[{n:>4}] type={:>3} keycode={:>3} flags={:?}",
            event.event_type(),
            event.keycode(),
            event.flags()
        );
        // Pass through — change to TapAction::Drop to swallow the event.
        TapAction::Pass
    }) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("\nFAILED to create tap: {e}\n");
            eprintln!("Grant Accessibility to: target/debug/examples/03_keyboard_tap");
            eprintln!("System Settings → Privacy & Security → Accessibility → +");
            return Ok(());
        }
    };
    println!("Listening for keyboard events. Press Ctrl-C to exit.");
    println!("(tap enabled = {})", tap.is_enabled());
    tap.run();
    Ok(())
}
