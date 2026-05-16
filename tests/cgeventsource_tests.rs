use cgevents::prelude::*;

#[test]
fn cgeventsource_setters_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    source.set_user_data(1234);
    source.set_pixels_per_line(11.25);
    source.set_local_events_filter(LocalEventsFilter::LOCAL_MOUSE, SuppressionState::Interval);

    assert!(source.source_state().is_none(), "private sources expose a unique runtime state ID");
    assert_eq!(source.user_data(), 1234);
    assert!((source.pixels_per_line() - 11.25).abs() < f64::EPSILON);
    assert_eq!(
        source.local_events_filter(SuppressionState::Interval),
        LocalEventsFilter::LOCAL_MOUSE
    );
    Ok(())
}
