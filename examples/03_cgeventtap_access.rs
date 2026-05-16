use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let taps = EventTap::installed()?;
    println!(
        "listen_access={} post_access={} taps={}",
        EventTap::preflight_listen_access(),
        EventTap::preflight_post_access(),
        taps.len()
    );
    Ok(())
}
