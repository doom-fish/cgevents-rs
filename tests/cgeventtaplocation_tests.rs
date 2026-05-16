use cgevents::prelude::*;

#[test]
fn cgeventtaplocation_bridge_values_match() {
    assert_eq!(unsafe { cgevents::ffi::cg_event_tap_location::cgevent_tap_location_raw_value(0) }, TapLocation::Hid.raw());
    assert_eq!(unsafe { cgevents::ffi::cg_event_tap_location::cgevent_tap_location_raw_value(1) }, TapLocation::Session.raw());
    assert_eq!(unsafe { cgevents::ffi::cg_event_tap_location::cgevent_tap_location_raw_value(2) }, TapLocation::AnnotatedSession.raw());
}
