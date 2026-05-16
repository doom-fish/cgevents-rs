//! Serialize, deserialize, and clone a Quartz event.
//!
//! Run: `cargo run --example 04_event_roundtrip`

use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = MouseEvent::move_to(Point::new(400.0, 300.0)).build(&source)?;
    event.set_timestamp(event.timestamp().saturating_add(1));

    let data = event.data()?;
    let roundtrip = Event::from_data(&data)?;
    let cloned = roundtrip.copy()?;

    println!("serialized bytes: {}", data.len());
    println!("type: {}", cloned.event_type());
    println!("location: {:?}", cloned.location());
    println!("unflipped location: {:?}", cloned.unflipped_location());

    Ok(())
}
