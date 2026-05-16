use cgevents::prelude::*;

#[test]
fn cgeventtap_access_helpers_and_inventory() -> Result<(), Box<dyn std::error::Error>> {
    let installed = EventTap::installed()?;
    assert!(installed.iter().all(|info| info.max_usec_latency >= info.min_usec_latency));
    let _ = EventTap::preflight_listen_access();
    let _ = EventTap::preflight_post_access();
    Ok(())
}
