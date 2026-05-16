use super::{CGEventBridgeHandle, CGEventSourceBridgeHandle};

unsafe extern "C" {
    pub fn cgevent_get_type_id() -> usize;
    pub fn cgevent_create(source: CGEventSourceBridgeHandle) -> CGEventBridgeHandle;
    pub fn cgevent_create_data_length(event: CGEventBridgeHandle) -> usize;
    pub fn cgevent_create_data_copy(
        event: CGEventBridgeHandle,
        buffer: *mut u8,
        bufferSize: usize,
    ) -> bool;
    pub fn cgevent_create_from_data(bytes: *const u8, length: usize) -> CGEventBridgeHandle;
    pub fn cgevent_create_mouse_event(
        source: CGEventSourceBridgeHandle,
        mouseType: u32,
        x: f64,
        y: f64,
        mouseButton: u32,
    ) -> CGEventBridgeHandle;
    pub fn cgevent_create_keyboard_event(
        source: CGEventSourceBridgeHandle,
        keycode: u16,
        keyDown: bool,
    ) -> CGEventBridgeHandle;
    pub fn cgevent_create_scroll_wheel_event(
        source: CGEventSourceBridgeHandle,
        units: u32,
        wheelCount: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> CGEventBridgeHandle;
    pub fn cgevent_create_copy(event: CGEventBridgeHandle) -> CGEventBridgeHandle;
    pub fn cgevent_create_source_from_event(event: CGEventBridgeHandle) -> CGEventSourceBridgeHandle;
    pub fn cgevent_set_source(event: CGEventBridgeHandle, source: CGEventSourceBridgeHandle);
    pub fn cgevent_get_type(event: CGEventBridgeHandle) -> u32;
    pub fn cgevent_set_type(event: CGEventBridgeHandle, r#type: u32);
    pub fn cgevent_get_timestamp(event: CGEventBridgeHandle) -> u64;
    pub fn cgevent_set_timestamp(event: CGEventBridgeHandle, timestamp: u64);
    pub fn cgevent_get_location(
        event: CGEventBridgeHandle,
        outX: *mut f64,
        outY: *mut f64,
    );
    pub fn cgevent_get_unflipped_location(
        event: CGEventBridgeHandle,
        outX: *mut f64,
        outY: *mut f64,
    );
    pub fn cgevent_set_location(event: CGEventBridgeHandle, x: f64, y: f64);
    pub fn cgevent_get_flags(event: CGEventBridgeHandle) -> u64;
    pub fn cgevent_set_flags(event: CGEventBridgeHandle, flags: u64);
    pub fn cgevent_get_integer_value_field(event: CGEventBridgeHandle, field: u32) -> i64;
    pub fn cgevent_set_integer_value_field(event: CGEventBridgeHandle, field: u32, value: i64);
    pub fn cgevent_get_double_value_field(event: CGEventBridgeHandle, field: u32) -> f64;
    pub fn cgevent_set_double_value_field(event: CGEventBridgeHandle, field: u32, value: f64);
    pub fn cgevent_keyboard_set_unicode_string(
        event: CGEventBridgeHandle,
        utf16: *const u16,
        length: usize,
    );
    pub fn cgevent_keyboard_get_unicode_string_length(event: CGEventBridgeHandle) -> usize;
    pub fn cgevent_keyboard_get_unicode_string(
        event: CGEventBridgeHandle,
        buffer: *mut u16,
        bufferLength: usize,
    ) -> bool;
    pub fn cgevent_post(event: CGEventBridgeHandle, tapLocation: u32);
    pub fn cgevent_post_to_pid(event: CGEventBridgeHandle, pid: i32);
    pub fn cgevent_release(event: CGEventBridgeHandle);
}
