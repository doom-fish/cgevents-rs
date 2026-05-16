use cgevents::prelude::*;

#[test]
fn cgeventfield_roundtrip_and_bridge_values() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(10.0, 20.0)).build(&source)?;
    event.set_integer_value(CGEventField::MouseEventDeltaX, 5);
    event.set_double_value(CGEventField::MouseEventPressure, 0.75);

    assert_eq!(event.integer_value(CGEventField::MouseEventDeltaX), 5);
    assert!(event.double_value(CGEventField::MouseEventPressure).is_finite());
    assert_eq!(unsafe { cgevents::ffi::cg_event_field::cgevent_field_raw_value(9) }, CGEventField::KeyboardEventKeycode.raw());
    assert!(CGEventField::MouseEventPressure.is_double_valued());
    Ok(())
}
