use cgevents::prelude::*;

#[test]
fn cgevent_roundtrip_data_copy_and_source() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    source.set_user_data(99);

    let event = KeyEvent::down(Keycode::A)
        .with_modifiers(ModifierFlags::SHIFT | ModifierFlags::COMMAND)
        .with_unicode("A")
        .build(&source)?;
    let data = event.data()?;
    let copy = Event::from_data(&data)?;
    assert!(copy.source().is_some(), "source from event should exist");

    assert_eq!(copy.keycode(), Keycode::A);
    assert_eq!(copy.flags(), ModifierFlags::SHIFT | ModifierFlags::COMMAND);
    assert_eq!(copy.unicode_string(), "A");
    Ok(())
}
