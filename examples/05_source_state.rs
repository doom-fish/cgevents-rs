//! Inspect and tune `CGEventSource` state.
//!
//! Run: `cargo run --example 05_source_state`

use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    source.set_pixels_per_line(12.5);
    source.set_user_data(42);
    source.set_local_events_filter(LocalEventsFilter::ALL, SuppressionState::Interval);

    println!("source_state_id: {}", source.source_state_id());
    println!("keyboard_type: {}", source.keyboard_type());
    println!("pixels_per_line: {:.1}", source.pixels_per_line());
    println!("user_data: {}", source.user_data());
    println!("flags_state: {:?}", source.flags_state());
    println!("a_is_down: {}", source.key_state(Keycode::A));
    println!(
        "left_mouse_down: {}",
        source.button_state(MouseButton::Left)
    );
    println!(
        "seconds_since_last_input: {:.3}",
        source.seconds_since_last_input_event()
    );
    println!(
        "key_down_count: {}",
        source.counter_for_event_type(cgevents::ffi::kCGEventKeyDown)
    );
    println!(
        "suppression_interval: {:.3}",
        source.local_events_suppression_interval()
    );

    Ok(())
}
