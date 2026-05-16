use cgevents::prelude::*;

#[test]
fn cgeventtype_bridge_values_and_masks_match() {
    let expected = [
        CGEventType::Null,
        CGEventType::LeftMouseDown,
        CGEventType::LeftMouseUp,
        CGEventType::RightMouseDown,
        CGEventType::RightMouseUp,
        CGEventType::MouseMoved,
        CGEventType::LeftMouseDragged,
        CGEventType::RightMouseDragged,
        CGEventType::KeyDown,
        CGEventType::KeyUp,
        CGEventType::FlagsChanged,
        CGEventType::ScrollWheel,
        CGEventType::TabletPointer,
        CGEventType::TabletProximity,
        CGEventType::OtherMouseDown,
        CGEventType::OtherMouseUp,
        CGEventType::OtherMouseDragged,
        CGEventType::TapDisabledByTimeout,
        CGEventType::TapDisabledByUserInput,
    ];

    for (index, event_type) in expected.into_iter().enumerate() {
        let index = u32::try_from(index).expect("event type index fits in u32");
        assert_eq!(unsafe { cgevents::ffi::cg_event_type::cgevent_type_raw_value(index) }, event_type.raw());
    }
    assert_eq!(unsafe { cgevents::ffi::cg_event_type::cgevent_type_mask_bit(CGEventType::KeyDown.raw()) }, CGEventType::KeyDown.mask_bit());
    assert_eq!(unsafe { cgevents::ffi::cg_event_type::cgevent_mask_for_all_events() }, CG_EVENT_MASK_FOR_ALL_EVENTS);
}
