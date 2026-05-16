use cgevents::prelude::*;

#[test]
fn cgeventtimestamp_roundtrip_and_bridge_size() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = Event::new(Some(&source))?;
    let timestamp = CGEventTimestamp::from_raw(0xDEAD_BEEF);
    event.set_event_timestamp(timestamp);

    assert_eq!(event.event_timestamp(), timestamp);
    assert_eq!(unsafe { cgevents::ffi::cg_event_timestamp::cgevent_timestamp_type_size() }, core::mem::size_of::<u64>());
    Ok(())
}
