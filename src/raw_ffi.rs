//! Raw Quartz Event Services C FFI.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::c_void;

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFRunLoopRef = *mut c_void;
pub type CFRunLoopSourceRef = *mut c_void;
pub type CFMachPortRef = *mut c_void;
pub type CFStringRef = *const c_void;
pub type CFIndex = isize;
pub type CFTypeID = usize;
pub type CFTimeInterval = f64;

pub type CGError = i32;
pub type CGEventRef = *mut c_void;
pub type CGEventSourceRef = *mut c_void;
pub type CGKeyCode = u16;
pub type CGCharCode = u16;
pub type CGButtonCount = u32;
pub type CGWheelCount = u32;
pub type CGEventTimestamp = u64;
pub type CGEventType = u32;
pub type CGEventField = u32;
pub type CGEventMouseSubtype = u32;
pub type CGEventTapLocation = u32;
pub type CGEventTapPlacement = u32;
pub type CGEventTapOptions = u32;
pub type CGEventMask = u64;
pub type CGEventSourceStateID = i32;
pub type CGEventSourceKeyboardType = u32;
pub type CGMouseButton = u32;
pub type CGScrollEventUnit = u32;
pub type CGGesturePhase = u32;
pub type CGMomentumScrollPhase = u32;
pub type CGScrollPhase = u32;
pub type CGEventFlags = u64;
pub type CGEventFilterMask = u32;
pub type CGEventSuppressionState = u32;
pub type CGEventTapProxy = *mut c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CGEventTapInformation {
    pub eventTapID: u32,
    pub tapPoint: CGEventTapLocation,
    pub options: CGEventTapOptions,
    pub eventsOfInterest: CGEventMask,
    pub tappingProcess: i32,
    pub processBeingTapped: i32,
    pub enabled: bool,
    pub minUsecLatency: f32,
    pub avgUsecLatency: f32,
    pub maxUsecLatency: f32,
}

pub const kCGErrorSuccess: CGError = 0;
pub const kCGEventNoErr: CGError = kCGErrorSuccess;

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
pub const kCGAnyInputEventType: CGEventType = u32::MAX;

pub const kCGMouseButtonLeft: CGMouseButton = 0;
pub const kCGMouseButtonRight: CGMouseButton = 1;
pub const kCGMouseButtonCenter: CGMouseButton = 2;

pub const kCGScrollEventUnitPixel: CGScrollEventUnit = 0;
pub const kCGScrollEventUnitLine: CGScrollEventUnit = 1;

pub const kCGMomentumScrollPhaseNone: CGMomentumScrollPhase = 0;
pub const kCGMomentumScrollPhaseBegin: CGMomentumScrollPhase = 1;
pub const kCGMomentumScrollPhaseContinue: CGMomentumScrollPhase = 2;
pub const kCGMomentumScrollPhaseEnd: CGMomentumScrollPhase = 3;

pub const kCGScrollPhaseBegan: CGScrollPhase = 1;
pub const kCGScrollPhaseChanged: CGScrollPhase = 2;
pub const kCGScrollPhaseEnded: CGScrollPhase = 4;
pub const kCGScrollPhaseCancelled: CGScrollPhase = 8;
pub const kCGScrollPhaseMayBegin: CGScrollPhase = 128;

pub const kCGGesturePhaseNone: CGGesturePhase = 0;
pub const kCGGesturePhaseBegan: CGGesturePhase = 1;
pub const kCGGesturePhaseChanged: CGGesturePhase = 2;
pub const kCGGesturePhaseEnded: CGGesturePhase = 4;
pub const kCGGesturePhaseCancelled: CGGesturePhase = 8;
pub const kCGGesturePhaseMayBegin: CGGesturePhase = 128;

pub const kCGEventFlagMaskAlphaShift: CGEventFlags = 0x0001_0000;
pub const kCGEventFlagMaskShift: CGEventFlags = 0x0002_0000;
pub const kCGEventFlagMaskControl: CGEventFlags = 0x0004_0000;
pub const kCGEventFlagMaskAlternate: CGEventFlags = 0x0008_0000;
pub const kCGEventFlagMaskCommand: CGEventFlags = 0x0010_0000;
pub const kCGEventFlagMaskNumericPad: CGEventFlags = 0x0020_0000;
pub const kCGEventFlagMaskHelp: CGEventFlags = 0x0040_0000;
pub const kCGEventFlagMaskSecondaryFn: CGEventFlags = 0x0080_0000;
pub const kCGEventFlagMaskNonCoalesced: CGEventFlags = 0x0000_0100;

pub const kCGEventMouseSubtypeDefault: CGEventMouseSubtype = 0;
pub const kCGEventMouseSubtypeTabletPoint: CGEventMouseSubtype = 1;
pub const kCGEventMouseSubtypeTabletProximity: CGEventMouseSubtype = 2;

pub const kCGHIDEventTap: CGEventTapLocation = 0;
pub const kCGSessionEventTap: CGEventTapLocation = 1;
pub const kCGAnnotatedSessionEventTap: CGEventTapLocation = 2;

pub const kCGHeadInsertEventTap: CGEventTapPlacement = 0;
pub const kCGTailAppendEventTap: CGEventTapPlacement = 1;

pub const kCGEventTapOptionDefault: CGEventTapOptions = 0;
pub const kCGEventTapOptionListenOnly: CGEventTapOptions = 1;

pub const kCGEventMaskForAllEvents: CGEventMask = !0;

pub const kCGNotifyEventTapAdded: &str = "com.apple.coregraphics.eventTapAdded";
pub const kCGNotifyEventTapRemoved: &str = "com.apple.coregraphics.eventTapRemoved";

pub const kCGEventSourceStatePrivate: CGEventSourceStateID = -1;
pub const kCGEventSourceStateCombinedSessionState: CGEventSourceStateID = 0;
pub const kCGEventSourceStateHIDSystemState: CGEventSourceStateID = 1;

pub const kCGEventFilterMaskPermitLocalMouseEvents: CGEventFilterMask = 0x0000_0001;
pub const kCGEventFilterMaskPermitLocalKeyboardEvents: CGEventFilterMask = 0x0000_0002;
pub const kCGEventFilterMaskPermitSystemDefinedEvents: CGEventFilterMask = 0x0000_0004;
pub const kCGEventFilterMaskPermitAllEvents: CGEventFilterMask = 0x0000_0007;

pub const kCGEventSuppressionStateSuppressionInterval: CGEventSuppressionState = 0;
pub const kCGEventSuppressionStateRemoteMouseDrag: CGEventSuppressionState = 1;

pub const kCGMouseEventNumber: CGEventField = 0;
pub const kCGMouseEventClickState: CGEventField = 1;
pub const kCGMouseEventPressure: CGEventField = 2;
pub const kCGMouseEventButtonNumber: CGEventField = 3;
pub const kCGMouseEventDeltaX: CGEventField = 4;
pub const kCGMouseEventDeltaY: CGEventField = 5;
pub const kCGMouseEventInstantMouser: CGEventField = 6;
pub const kCGMouseEventSubtype: CGEventField = 7;
pub const kCGKeyboardEventAutorepeat: CGEventField = 8;
pub const kCGKeyboardEventKeycode: CGEventField = 9;
pub const kCGKeyboardEventKeyboardType: CGEventField = 10;
pub const kCGScrollWheelEventDeltaAxis1: CGEventField = 11;
pub const kCGScrollWheelEventDeltaAxis2: CGEventField = 12;
pub const kCGScrollWheelEventDeltaAxis3: CGEventField = 13;
pub const kCGScrollWheelEventInstantMouser: CGEventField = 14;
pub const kCGTabletEventPointX: CGEventField = 15;
pub const kCGTabletEventPointY: CGEventField = 16;
pub const kCGTabletEventPointZ: CGEventField = 17;
pub const kCGTabletEventPointButtons: CGEventField = 18;
pub const kCGTabletEventPointPressure: CGEventField = 19;
pub const kCGTabletEventTiltX: CGEventField = 20;
pub const kCGTabletEventTiltY: CGEventField = 21;
pub const kCGTabletEventRotation: CGEventField = 22;
pub const kCGTabletEventTangentialPressure: CGEventField = 23;
pub const kCGTabletEventDeviceID: CGEventField = 24;
pub const kCGTabletEventVendor1: CGEventField = 25;
pub const kCGTabletEventVendor2: CGEventField = 26;
pub const kCGTabletEventVendor3: CGEventField = 27;
pub const kCGTabletProximityEventVendorID: CGEventField = 28;
pub const kCGTabletProximityEventTabletID: CGEventField = 29;
pub const kCGTabletProximityEventPointerID: CGEventField = 30;
pub const kCGTabletProximityEventDeviceID: CGEventField = 31;
pub const kCGTabletProximityEventSystemTabletID: CGEventField = 32;
pub const kCGTabletProximityEventVendorPointerType: CGEventField = 33;
pub const kCGTabletProximityEventVendorPointerSerialNumber: CGEventField = 34;
pub const kCGTabletProximityEventVendorUniqueID: CGEventField = 35;
pub const kCGTabletProximityEventCapabilityMask: CGEventField = 36;
pub const kCGTabletProximityEventPointerType: CGEventField = 37;
pub const kCGTabletProximityEventEnterProximity: CGEventField = 38;
pub const kCGEventTargetProcessSerialNumber: CGEventField = 39;
pub const kCGEventTargetUnixProcessID: CGEventField = 40;
pub const kCGEventSourceUnixProcessID: CGEventField = 41;
pub const kCGEventSourceUserData: CGEventField = 42;
pub const kCGEventSourceUserID: CGEventField = 43;
pub const kCGEventSourceGroupID: CGEventField = 44;
pub const kCGEventSourceStateID: CGEventField = 45;
pub const kCGScrollWheelEventIsContinuous: CGEventField = 88;
pub const kCGMouseEventWindowUnderMousePointer: CGEventField = 91;
pub const kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent: CGEventField = 92;
pub const kCGScrollWheelEventFixedPtDeltaAxis1: CGEventField = 93;
pub const kCGScrollWheelEventFixedPtDeltaAxis2: CGEventField = 94;
pub const kCGScrollWheelEventFixedPtDeltaAxis3: CGEventField = 95;
pub const kCGScrollWheelEventPointDeltaAxis1: CGEventField = 96;
pub const kCGScrollWheelEventPointDeltaAxis2: CGEventField = 97;
pub const kCGScrollWheelEventPointDeltaAxis3: CGEventField = 98;
pub const kCGScrollWheelEventScrollPhase: CGEventField = 99;
pub const kCGScrollWheelEventScrollCount: CGEventField = 100;
pub const kCGScrollWheelEventMomentumPhase: CGEventField = 123;
pub const kCGEventUnacceleratedPointerMovementX: CGEventField = 170;
pub const kCGEventUnacceleratedPointerMovementY: CGEventField = 171;
pub const kCGScrollWheelEventMomentumOptionPhase: CGEventField = 173;
pub const kCGScrollWheelEventAcceleratedDeltaAxis2: CGEventField = 175;
pub const kCGScrollWheelEventAcceleratedDeltaAxis1: CGEventField = 176;
pub const kCGScrollWheelEventRawDeltaAxis2: CGEventField = 177;
pub const kCGScrollWheelEventRawDeltaAxis1: CGEventField = 178;

pub type CGEventTapCallBack = unsafe extern "C" fn(
    proxy: CGEventTapProxy,
    ty: CGEventType,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef;

unsafe extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFRunLoopCommonModes: CFStringRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
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

    pub fn CGEventGetTypeID() -> CFTypeID;
    pub fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    pub fn CGEventCreateData(allocator: CFAllocatorRef, event: CGEventRef) -> CFDataRef;
    pub fn CGEventCreateFromData(allocator: CFAllocatorRef, data: CFDataRef) -> CGEventRef;
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
    ) -> CGEventRef;
    pub fn CGEventCreateScrollWheelEvent2(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        wheel_count: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> CGEventRef;
    pub fn CGEventCreateCopy(event: CGEventRef) -> CGEventRef;
    pub fn CGEventCreateSourceFromEvent(event: CGEventRef) -> CGEventSourceRef;
    pub fn CGEventSetSource(event: CGEventRef, source: CGEventSourceRef);
    pub fn CGEventGetType(event: CGEventRef) -> CGEventType;
    pub fn CGEventSetType(event: CGEventRef, ty: CGEventType);
    pub fn CGEventGetTimestamp(event: CGEventRef) -> CGEventTimestamp;
    pub fn CGEventSetTimestamp(event: CGEventRef, timestamp: CGEventTimestamp);
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    pub fn CGEventGetUnflippedLocation(event: CGEventRef) -> CGPoint;
    pub fn CGEventSetLocation(event: CGEventRef, location: CGPoint);
    pub fn CGEventGetFlags(event: CGEventRef) -> CGEventFlags;
    pub fn CGEventSetFlags(event: CGEventRef, flags: CGEventFlags);
    pub fn CGEventKeyboardGetUnicodeString(
        event: CGEventRef,
        max_string_length: usize,
        actual_string_length: *mut usize,
        unicode_string: *mut u16,
    );
    pub fn CGEventKeyboardSetUnicodeString(
        event: CGEventRef,
        string_length: usize,
        unicode_string: *const u16,
    );
    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: CGEventField) -> i64;
    pub fn CGEventSetIntegerValueField(event: CGEventRef, field: CGEventField, value: i64);
    pub fn CGEventGetDoubleValueField(event: CGEventRef, field: CGEventField) -> f64;
    pub fn CGEventSetDoubleValueField(event: CGEventRef, field: CGEventField, value: f64);
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: CGEventMask,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapCreateForPSN(
        process_serial_number: *mut c_void,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: CGEventMask,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapCreateForPid(
        pid: i32,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: CGEventMask,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CGEventTapIsEnabled(tap: CFMachPortRef) -> bool;
    pub fn CGEventTapPostEvent(proxy: CGEventTapProxy, event: CGEventRef);
    pub fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
    pub fn CGEventPostToPSN(process_serial_number: *mut c_void, event: CGEventRef);
    pub fn CGEventPostToPid(pid: i32, event: CGEventRef);
    pub fn CGGetEventTapList(
        max_number_of_taps: u32,
        tap_list: *mut CGEventTapInformation,
        event_tap_count: *mut u32,
    ) -> CGError;
    pub fn CGPreflightListenEventAccess() -> bool;
    pub fn CGRequestListenEventAccess() -> bool;
    pub fn CGPreflightPostEventAccess() -> bool;
    pub fn CGRequestPostEventAccess() -> bool;

    pub fn CGEventSourceGetTypeID() -> CFTypeID;
    pub fn CGEventSourceCreate(state_id: CGEventSourceStateID) -> CGEventSourceRef;
    pub fn CGEventSourceGetKeyboardType(source: CGEventSourceRef) -> CGEventSourceKeyboardType;
    pub fn CGEventSourceSetKeyboardType(
        source: CGEventSourceRef,
        keyboard_type: CGEventSourceKeyboardType,
    );
    pub fn CGEventSourceGetPixelsPerLine(source: CGEventSourceRef) -> f64;
    pub fn CGEventSourceSetPixelsPerLine(source: CGEventSourceRef, pixels_per_line: f64);
    pub fn CGEventSourceGetSourceStateID(source: CGEventSourceRef) -> CGEventSourceStateID;
    pub fn CGEventSourceButtonState(state_id: CGEventSourceStateID, button: CGMouseButton) -> bool;
    pub fn CGEventSourceKeyState(state_id: CGEventSourceStateID, key: CGKeyCode) -> bool;
    pub fn CGEventSourceFlagsState(state_id: CGEventSourceStateID) -> CGEventFlags;
    pub fn CGEventSourceSecondsSinceLastEventType(
        state_id: CGEventSourceStateID,
        event_type: CGEventType,
    ) -> CFTimeInterval;
    pub fn CGEventSourceCounterForEventType(
        state_id: CGEventSourceStateID,
        event_type: CGEventType,
    ) -> u32;
    pub fn CGEventSourceSetUserData(source: CGEventSourceRef, user_data: i64);
    pub fn CGEventSourceGetUserData(source: CGEventSourceRef) -> i64;
    pub fn CGEventSourceSetLocalEventsFilterDuringSuppressionState(
        source: CGEventSourceRef,
        filter: CGEventFilterMask,
        state: CGEventSuppressionState,
    );
    pub fn CGEventSourceGetLocalEventsFilterDuringSuppressionState(
        source: CGEventSourceRef,
        state: CGEventSuppressionState,
    ) -> CGEventFilterMask;
    pub fn CGEventSourceSetLocalEventsSuppressionInterval(source: CGEventSourceRef, seconds: f64);
    pub fn CGEventSourceGetLocalEventsSuppressionInterval(source: CGEventSourceRef) -> f64;
}

#[must_use]
pub const fn cg_event_mask_bit(ty: CGEventType) -> CGEventMask {
    if ty < 64 {
        1_u64 << ty
    } else {
        0
    }
}
