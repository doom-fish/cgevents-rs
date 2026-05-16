use cgevents::prelude::*;

fn main() {
    let key_down_mask = CGEventType::KeyDown.mask_bit();
    let mouse_moved_mask = CGEventType::MouseMoved.mask_bit();
    let bridge_key_down_mask = unsafe { cgevents::ffi::cg_event_type::cgevent_type_mask_bit(CGEventType::KeyDown.raw()) };

    assert_eq!(key_down_mask, bridge_key_down_mask);
    assert_ne!(key_down_mask, mouse_moved_mask);
    assert_eq!(CG_EVENT_MASK_FOR_ALL_EVENTS, unsafe {
        cgevents::ffi::cg_event_type::cgevent_mask_for_all_events()
    });
    println!("key_down_mask={key_down_mask:#x} mouse_moved_mask={mouse_moved_mask:#x}");
}
