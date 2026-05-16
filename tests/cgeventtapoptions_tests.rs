use cgevents::prelude::*;

#[test]
fn cgeventtapoptions_bridge_values_match() {
    assert_eq!(unsafe { cgevents::ffi::cg_event_tap_options::cgevent_tap_options_raw_value(0) }, CGEventTapOptions::Default.raw());
    assert_eq!(unsafe { cgevents::ffi::cg_event_tap_options::cgevent_tap_options_raw_value(1) }, CGEventTapOptions::ListenOnly.raw());
}
