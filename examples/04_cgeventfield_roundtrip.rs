use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(12.0, 24.0)).build(&source)?;
    event.set_integer_value(CGEventField::MouseEventDeltaX, 7);
    event.set_integer_value(CGEventField::MouseEventDeltaY, -3);
    event.set_double_value(CGEventField::MouseEventPressure, 0.5);

    assert_eq!(event.integer_value(CGEventField::MouseEventDeltaX), 7);
    assert_eq!(event.integer_value(CGEventField::MouseEventDeltaY), -3);
    assert!(event.double_value(CGEventField::MouseEventPressure).is_finite());
    println!("field_roundtrip_ok=true");
    Ok(())
}
