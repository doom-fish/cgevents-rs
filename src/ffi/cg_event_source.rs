use super::CGEventSourceBridgeHandle;

unsafe extern "C" {
    pub fn cgevent_source_get_type_id() -> usize;
    pub fn cgevent_source_create(stateID: i32) -> CGEventSourceBridgeHandle;
    pub fn cgevent_source_get_keyboard_type(source: CGEventSourceBridgeHandle) -> u32;
    pub fn cgevent_source_set_keyboard_type(source: CGEventSourceBridgeHandle, keyboardType: u32);
    pub fn cgevent_source_get_pixels_per_line(source: CGEventSourceBridgeHandle) -> f64;
    pub fn cgevent_source_set_pixels_per_line(source: CGEventSourceBridgeHandle, pixelsPerLine: f64);
    pub fn cgevent_source_get_source_state_id(source: CGEventSourceBridgeHandle) -> i32;
    pub fn cgevent_source_button_state(stateID: i32, button: u32) -> bool;
    pub fn cgevent_source_key_state(stateID: i32, keycode: u16) -> bool;
    pub fn cgevent_source_flags_state(stateID: i32) -> u64;
    pub fn cgevent_source_seconds_since_last_event_type(stateID: i32, eventType: u32) -> f64;
    pub fn cgevent_source_counter_for_event_type(stateID: i32, eventType: u32) -> u32;
    pub fn cgevent_source_set_user_data(source: CGEventSourceBridgeHandle, userData: i64);
    pub fn cgevent_source_get_user_data(source: CGEventSourceBridgeHandle) -> i64;
    pub fn cgevent_source_set_local_events_filter_during_suppression_state(
        source: CGEventSourceBridgeHandle,
        filter: u32,
        state: u32,
    );
    pub fn cgevent_source_get_local_events_filter_during_suppression_state(
        source: CGEventSourceBridgeHandle,
        state: u32,
    ) -> u32;
    pub fn cgevent_source_set_local_events_suppression_interval(
        source: CGEventSourceBridgeHandle,
        seconds: f64,
    );
    pub fn cgevent_source_get_local_events_suppression_interval(
        source: CGEventSourceBridgeHandle,
    ) -> f64;
    pub fn cgevent_source_release(source: CGEventSourceBridgeHandle);
}
