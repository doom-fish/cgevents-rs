//! Raw FFI declarations for the subset of Quartz Event Services we use.
//!
//! Pure C — no Swift bridge needed. Linked against `CoreGraphics` +
//! `CoreFoundation` + `ApplicationServices`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]

use core::ffi::c_void;

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFRunLoopRef = *mut c_void;
pub type CFRunLoopSourceRef = *mut c_void;
pub type CFMachPortRef = *mut c_void;
pub type CFStringRef = *const c_void;
pub type CFIndex = isize;

pub type CGEventRef = *mut c_void;
pub type CGEventSourceRef = *mut c_void;
pub type CGKeyCode = u16;
pub type CGEventTimestamp = u64;
pub type CGEventType = u32;
pub type CGEventField = u32;
pub type CGEventTapLocation = u32;
pub type CGEventTapPlacement = u32;
pub type CGEventTapOptions = u32;
pub type CGEventMask = u64;
pub type CGEventSourceStateID = i32;
pub type CGMouseButton = u32;
pub type CGScrollEventUnit = u32;
pub type CGEventFlags = u64;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

// ---- Event types ----

pub const kCGEventNull: CGEventType = 0;
pub const kCGEventLeftMouseDown: CGEventType = 1;
pub const kCGEventLeftMouseUp: CGEventType = 2;
pub const kCGEventRightMouseDown: CGEventType = 3;
pub const kCGEventRightMouseUp: CGEventType = 4;
pub const kCGEventMouseMoved: CGEventType = 5;
pub const kCGEventLeftMouseDragged: CGEventType = 6;
pub const kCGEventRightMouseDragged: CGEventType = 7;
pub const kCGEventKeyDown: CGEventType = 10;
pub const kCGEventKeyUp: CGEventType = 11;
pub const kCGEventFlagsChanged: CGEventType = 12;
pub const kCGEventScrollWheel: CGEventType = 22;
pub const kCGEventTabletPointer: CGEventType = 23;
pub const kCGEventTabletProximity: CGEventType = 24;
pub const kCGEventOtherMouseDown: CGEventType = 25;
pub const kCGEventOtherMouseUp: CGEventType = 26;
pub const kCGEventOtherMouseDragged: CGEventType = 27;
pub const kCGEventTapDisabledByTimeout: CGEventType = 0xFFFF_FFFE;
pub const kCGEventTapDisabledByUserInput: CGEventType = 0xFFFF_FFFF;

// ---- Mouse buttons ----

pub const kCGMouseButtonLeft: CGMouseButton = 0;
pub const kCGMouseButtonRight: CGMouseButton = 1;
pub const kCGMouseButtonCenter: CGMouseButton = 2;

// ---- Scroll units ----

pub const kCGScrollEventUnitPixel: CGScrollEventUnit = 0;
pub const kCGScrollEventUnitLine: CGScrollEventUnit = 1;

// ---- Modifier flags (subset) ----

pub const kCGEventFlagMaskAlphaShift: CGEventFlags = 0x0001_0000;
pub const kCGEventFlagMaskShift: CGEventFlags = 0x0002_0000;
pub const kCGEventFlagMaskControl: CGEventFlags = 0x0004_0000;
pub const kCGEventFlagMaskAlternate: CGEventFlags = 0x0008_0000;
pub const kCGEventFlagMaskCommand: CGEventFlags = 0x0010_0000;
pub const kCGEventFlagMaskHelp: CGEventFlags = 0x0040_0000;
pub const kCGEventFlagMaskSecondaryFn: CGEventFlags = 0x0080_0000;
pub const kCGEventFlagMaskNumericPad: CGEventFlags = 0x0020_0000;

// ---- Tap location + options ----

pub const kCGHIDEventTap: CGEventTapLocation = 0;
pub const kCGSessionEventTap: CGEventTapLocation = 1;
pub const kCGAnnotatedSessionEventTap: CGEventTapLocation = 2;

pub const kCGHeadInsertEventTap: CGEventTapPlacement = 0;
pub const kCGTailAppendEventTap: CGEventTapPlacement = 1;

pub const kCGEventTapOptionDefault: CGEventTapOptions = 0;
pub const kCGEventTapOptionListenOnly: CGEventTapOptions = 1;

// ---- Event-source state ----

pub const kCGEventSourceStatePrivate: CGEventSourceStateID = -1;
pub const kCGEventSourceStateCombinedSessionState: CGEventSourceStateID = 0;
pub const kCGEventSourceStateHIDSystemState: CGEventSourceStateID = 1;

pub type CGEventTapCallBack = unsafe extern "C" fn(
    proxy: *mut c_void,
    ty: CGEventType,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopRun();
    pub fn CFRunLoopStop(rl: CFRunLoopRef);
    pub fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        port: CFMachPortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
    pub fn CFMachPortInvalidate(port: CFMachPortRef);

    // CGEventSource
    pub fn CGEventSourceCreate(state_id: CGEventSourceStateID) -> CGEventSourceRef;

    // CGEvent — create
    pub fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    pub fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouse_type: CGEventType,
        mouse_cursor_position: CGPoint,
        mouse_button: CGMouseButton,
    ) -> CGEventRef;
    pub fn CGEventCreateKeyboardEvent(
        source: CGEventSourceRef,
        keycode: CGKeyCode,
        key_down: bool,
    ) -> CGEventRef;
    pub fn CGEventCreateScrollWheelEvent(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        wheel_count: u32,
        wheel1: i32,
        // wheel2/3 intentionally elided — pass 0 explicitly via wrapper.
    ) -> CGEventRef;

    // CGEvent — accessors
    pub fn CGEventGetType(event: CGEventRef) -> CGEventType;
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    pub fn CGEventSetLocation(event: CGEventRef, location: CGPoint);
    pub fn CGEventGetFlags(event: CGEventRef) -> CGEventFlags;
    pub fn CGEventSetFlags(event: CGEventRef, flags: CGEventFlags);
    pub fn CGEventGetTimestamp(event: CGEventRef) -> CGEventTimestamp;
    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: CGEventField) -> i64;
    pub fn CGEventSetIntegerValueField(event: CGEventRef, field: CGEventField, value: i64);
    pub fn CGEventKeyboardSetUnicodeString(
        event: CGEventRef,
        string_length: usize,
        unicode_string: *const u16,
    );
    pub fn CGEventKeyboardGetUnicodeString(
        event: CGEventRef,
        max_string_length: usize,
        actual_string_length: *mut usize,
        unicode_string: *mut u16,
    );

    // CGEvent — post
    pub fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
    pub fn CGEventPostToPid(pid: i32, event: CGEventRef);

    // CGEventTap
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: CGEventMask,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CGEventTapIsEnabled(tap: CFMachPortRef) -> bool;
}

/// Build a `CGEventMask` covering the given event types.
#[must_use]
pub const fn cg_event_mask_bit(ty: CGEventType) -> CGEventMask {
    1u64 << ty
}

/// `kCGEventFieldKeyboardEventKeycode` — read with `CGEventGetIntegerValueField`.
pub const kCGKeyboardEventKeycode: CGEventField = 9;
/// `kCGScrollWheelEventDeltaAxis1` field id.
pub const kCGScrollWheelEventDeltaAxis1: CGEventField = 11;
/// `kCGScrollWheelEventDeltaAxis2` field id.
pub const kCGScrollWheelEventDeltaAxis2: CGEventField = 12;
