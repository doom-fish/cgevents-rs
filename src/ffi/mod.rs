//! Swift-bridge FFI declarations for Quartz Event Services.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::c_void;

pub use apple_cf::raw::{
    CFAllocatorRef, CFDataRef, CFMachPortRef, CFRunLoopRef, CFRunLoopSourceRef, CFStringRef,
    CFTypeRef,
};

pub type CGEventBridgeHandle = *mut c_void;
pub type CGEventSourceBridgeHandle = *mut c_void;
pub type CGEventTapBridgeHandle = *mut c_void;
pub type CGEventTapProxyBridgeHandle = *mut c_void;

pub use apple_cf::cg::CGPoint;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CGEventTapInformation {
    pub event_tap_id: u32,
    pub tap_point: u32,
    pub options: u32,
    pub events_of_interest: u64,
    pub tapping_process: i32,
    pub process_being_tapped: i32,
    pub enabled: bool,
    pub min_usec_latency: f32,
    pub avg_usec_latency: f32,
    pub max_usec_latency: f32,
}

// MARK: - ABI Layout Assertions
//
// `CGEventTapInformation` is filled in by the Swift bridge (as
// `FFIEventTapInformation` in `swift-bridge/Sources/CGEventsBridge/Support.swift`)
// into a packed buffer that Rust reads back by value in
// `EventTap::installed`. If the size/alignment of either side ever drifts —
// e.g. a field is reordered, retyped, or the `bool`/`f32` padding changes — the
// marshalled data silently corrupts. These compile-time assertions pin the
// Rust ABI; the cross-language `cgevent_verify_ffi_layout` check (verified by
// `tests/ffi_layout_tests.rs`) guards that the Swift layout still agrees.
//
// NOTE: `offset_of!` is deliberately not used here — the crate's MSRV is 1.76
// and `offset_of!` only stabilised in 1.77. Size and alignment assertions are
// MSRV-safe and still catch field reordering/retyping (which changes padding
// and therefore the overall size).
use core::mem::{align_of, size_of};

const _: () = assert!(size_of::<CGEventTapInformation>() == 48);
const _: () = assert!(align_of::<CGEventTapInformation>() == 8);

extern "C" {
    /// Cross-language ABI check implemented in the Swift bridge.
    ///
    /// Returns `true` only if the Swift `MemoryLayout` (size, stride and
    /// alignment) of `FFIEventTapInformation` matches the values pinned on the
    /// Rust side for [`CGEventTapInformation`]. Verified by
    /// `tests/ffi_layout_tests.rs`.
    pub fn cgevent_verify_ffi_layout() -> bool;
}

pub mod cg_event;
pub mod cg_event_field;
pub mod cg_event_flags;
pub mod cg_event_mouse_subtype;
pub mod cg_event_source;
pub mod cg_event_tap;
pub mod cg_event_tap_location;
pub mod cg_event_tap_options;
pub mod cg_event_tap_proxy;
pub mod cg_event_timestamp;
pub mod cg_event_type;
pub mod cg_gesture_phase;
pub mod cg_momentum_scroll_phase;
pub mod cg_scroll_phase;
