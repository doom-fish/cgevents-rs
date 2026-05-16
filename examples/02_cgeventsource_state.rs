use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    source.set_user_data(42);
    source.set_pixels_per_line(12.5);
    source.set_local_events_filter(LocalEventsFilter::LOCAL_KEYBOARD, SuppressionState::Interval);

    assert_eq!(source.user_data(), 42);
    assert!((source.pixels_per_line() - 12.5).abs() < f64::EPSILON);
    assert_eq!(
        source.local_events_filter(SuppressionState::Interval),
        LocalEventsFilter::LOCAL_KEYBOARD
    );
    println!("state_id={} keyboard_type={}", source.source_state_id(), source.keyboard_type());
    Ok(())
}
