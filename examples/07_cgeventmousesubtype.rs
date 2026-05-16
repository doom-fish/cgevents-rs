use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(1.0, 2.0)).build(&source)?;
    event.set_mouse_subtype(CGEventMouseSubtype::TabletPoint);

    assert_eq!(event.mouse_subtype(), Some(CGEventMouseSubtype::TabletPoint));
    println!("mouse_subtype={}", CGEventMouseSubtype::TabletPoint.raw());
    Ok(())
}
