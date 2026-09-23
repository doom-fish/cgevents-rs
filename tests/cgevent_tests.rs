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

#[test]
fn unicode_strings_are_limited_to_twenty_utf16_units() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(MAX_UNICODE_STRING_LENGTH, 20);
    let source = EventSource::private()?;
    let event = KeyEvent::down(Keycode::A).build(&source)?;

    let twenty_letters = "abcdefghijklmnopqrst";
    event.set_unicode_string(twenty_letters)?;
    assert_eq!(event.unicode_string(), twenty_letters);

    let twenty_units = "\u{1F600}".repeat(10);
    event.set_unicode_string(&twenty_units)?;
    assert_eq!(event.unicode_string(), twenty_units);

    assert!(matches!(
        event.set_unicode_string("abcdefghijklmnopqrstu"),
        Err(CGError::InvalidArgument(_))
    ));
    assert_eq!(event.unicode_string(), twenty_units);

    let too_long = KeyEvent::down(Keycode::A)
        .with_unicode(&"\u{1F600}".repeat(11))
        .build(&source);
    assert!(matches!(too_long, Err(CGError::InvalidArgument(_))));
    Ok(())
}
