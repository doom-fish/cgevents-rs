use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(0.0, 0.0)).build(&source)?;
    event.post(TapLocation::Session);

    assert_eq!(TapLocation::Session.raw(), 1);
    assert_eq!(TapLocation::from_raw(2), Some(TapLocation::AnnotatedSession));
    println!("posted_to_session=true");
    Ok(())
}
