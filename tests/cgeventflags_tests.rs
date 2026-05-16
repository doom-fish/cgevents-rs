use cgevents::prelude::*;

#[test]
fn cgeventflags_bridge_values_match() {
    let expected = [
        ModifierFlags::ALPHA_SHIFT,
        ModifierFlags::SHIFT,
        ModifierFlags::CONTROL,
        ModifierFlags::ALTERNATE,
        ModifierFlags::COMMAND,
        ModifierFlags::HELP,
        ModifierFlags::SECONDARY_FN,
        ModifierFlags::NUMERIC_PAD,
        ModifierFlags::NON_COALESCED,
    ];

    for (index, flag) in expected.into_iter().enumerate() {
        let index = u32::try_from(index).expect("flag index fits in u32");
        assert_eq!(unsafe { cgevents::ffi::cg_event_flags::cgevent_flags_raw_value(index) }, flag.bits());
    }
    assert_eq!(unsafe { cgevents::ffi::cg_event_flags::cgevent_flags_known_mask() }, cgevents::CG_EVENT_FLAGS_KNOWN_MASK);
}
