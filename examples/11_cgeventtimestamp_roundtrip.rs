use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = Event::new(Some(&source))?;
    let timestamp = CGEventTimestamp::from_raw(123_456_789);
    event.set_event_timestamp(timestamp);

    assert_eq!(event.event_timestamp(), timestamp);
    assert_eq!(unsafe { cgevents::ffi::cg_event_timestamp::cgevent_timestamp_type_size() }, core::mem::size_of::<u64>());
    println!("timestamp={}", event.timestamp());
    Ok(())
}
