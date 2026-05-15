//! Synthesise a left mouse click at a screen coordinate.
//!
//! Run: `cargo run --example 01_synth_click`
//!
//! No permission required (`CGEventPost` works without Accessibility grant).
//! The cursor will visibly jump to the target point and click.

use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = Point::new(960.0, 540.0); // middle of a 1920×1080 screen

    println!("Moving cursor to {target:?}");
    MouseEvent::move_to(target).post(TapLocation::Session)?;
    std::thread::sleep(std::time::Duration::from_millis(200));

    println!("Clicking left button");
    MouseEvent::button_down(target, MouseButton::Left).post(TapLocation::Session)?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    MouseEvent::button_up(target, MouseButton::Left).post(TapLocation::Session)?;

    println!("OK click synthesised at {target:?}");
    Ok(())
}
