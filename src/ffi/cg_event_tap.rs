use core::ffi::c_void;

use super::{CGEventTapBridgeHandle, CGEventTapProxyBridgeHandle};

pub type RustTapCallback = unsafe extern "C" fn(
    context: *mut c_void,
    proxy: CGEventTapProxyBridgeHandle,
    event_type: u32,
    event: *mut c_void,
) -> i32;

/// C trampoline the Swift bridge calls to take a +1 reference on the Rust tap
/// context, keeping it alive for the lifetime of the Swift `EventTapHolder`.
pub type ContextRetainCallback = extern "C" fn(context: *mut c_void);

/// C trampoline the Swift bridge calls from `EventTapHolder.deinit` to drop the
/// +1 reference taken via [`ContextRetainCallback`].
pub type ContextReleaseCallback = extern "C" fn(context: *mut c_void);

unsafe extern "C" {
    pub fn cgevent_tap_create(
        location: u32,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: RustTapCallback,
        context: *mut c_void,
        contextRetain: ContextRetainCallback,
        contextRelease: ContextReleaseCallback,
    ) -> CGEventTapBridgeHandle;
    pub fn cgevent_tap_create_for_pid(
        pid: i32,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: RustTapCallback,
        context: *mut c_void,
        contextRetain: ContextRetainCallback,
        contextRelease: ContextReleaseCallback,
    ) -> CGEventTapBridgeHandle;
    pub fn cgevent_tap_enable(tap: CGEventTapBridgeHandle, enable: bool);
    pub fn cgevent_tap_is_enabled(tap: CGEventTapBridgeHandle) -> bool;
    pub fn cgevent_tap_run_current_run_loop();
    pub fn cgevent_tap_stop_current_run_loop();
    pub fn cgevent_tap_stop(tap: CGEventTapBridgeHandle);
    pub fn cgevent_tap_release(tap: CGEventTapBridgeHandle);
    pub fn cgevent_get_event_tap_list(
        maxNumberOfTaps: u32,
        tapList: *mut c_void,
        eventTapCount: *mut u32,
    ) -> i32;
    pub fn cgevent_preflight_listen_event_access() -> bool;
    pub fn cgevent_request_listen_event_access() -> bool;
    pub fn cgevent_preflight_post_event_access() -> bool;
    pub fn cgevent_request_post_event_access() -> bool;
}
