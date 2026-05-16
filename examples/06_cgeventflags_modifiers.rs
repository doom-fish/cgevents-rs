use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = ModifierFlags::COMMAND | ModifierFlags::SHIFT | ModifierFlags::SECONDARY_FN;
    let source = EventSource::private()?;
    let event = KeyEvent::down(Keycode::A).with_modifiers(flags).build(&source)?;

    assert_eq!(event.flags(), flags);
    assert_eq!(unsafe { cgevents::ffi::cg_event_flags::cgevent_flags_known_mask() }, cgevents::CG_EVENT_FLAGS_KNOWN_MASK);
    println!("flags={:#x}", event.flags().bits());
    Ok(())
}
