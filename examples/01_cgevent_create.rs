use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = KeyEvent::down(Keycode::A)
        .with_modifiers(ModifierFlags::SHIFT)
        .with_unicode("A")
        .build(&source)?;
    let data = event.data()?;
    let roundtrip = Event::from_data(&data)?;

    assert_eq!(roundtrip.keycode(), Keycode::A);
    assert_eq!(roundtrip.flags(), ModifierFlags::SHIFT);
    assert_eq!(roundtrip.unicode_string(), "A");
    println!("event_type={} bytes={}", roundtrip.event_type(), data.len());
    Ok(())
}
