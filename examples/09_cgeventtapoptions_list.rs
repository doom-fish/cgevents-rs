use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let taps = EventTap::installed()?;
    assert_eq!(CGEventTapOptions::Default.raw(), 0);
    assert_eq!(CGEventTapOptions::ListenOnly.raw(), 1);
    println!("taps={} default={} listen_only={}", taps.len(), CGEventTapOptions::Default.raw(), CGEventTapOptions::ListenOnly.raw());
    Ok(())
}
