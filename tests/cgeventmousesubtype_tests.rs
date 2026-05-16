use cgevents::prelude::*;

#[test]
fn cgeventmousesubtype_roundtrip_and_bridge_values() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(0.0, 0.0)).build(&source)?;
    event.set_mouse_subtype(CGEventMouseSubtype::TabletPoint);

    assert_eq!(event.mouse_subtype(), Some(CGEventMouseSubtype::TabletPoint));
    assert_eq!(unsafe { cgevents::ffi::cg_event_mouse_subtype::cgevent_mouse_subtype_raw_value(0) }, CGEventMouseSubtype::Default.raw());
    assert_eq!(unsafe { cgevents::ffi::cg_event_mouse_subtype::cgevent_mouse_subtype_raw_value(1) }, CGEventMouseSubtype::TabletPoint.raw());
    Ok(())
}
