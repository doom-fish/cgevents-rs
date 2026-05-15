//! Type a string using `type_string`. Works for plain text + unicode +
//! emoji (bypasses the keymap by attaching a unicode payload to each
//! keyboard event).
//!
//! Run: `cargo run --example 02_type_string`
//!
//! Tip: focus a text field (e.g. `TextEdit`, Notes, terminal) within ~3
//! seconds of starting this example and you'll see the typed output.

use cgevents::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Typing in 3 seconds — focus a text field now…");
    for i in (1..=3).rev() {
        println!("  {i}");
        thread::sleep(Duration::from_secs(1));
    }

    let text = "Hello from cgevents!\nUnicode: 🌍 世界 é ñ\n";
    println!("Typing: {text:?}");
    type_string(text, TapLocation::Session)?;
    println!("Done");
    Ok(())
}
