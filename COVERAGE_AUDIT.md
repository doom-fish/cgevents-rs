# cgevents coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 210
VERIFIED: 191
GAPS: 18
EXEMPT: 1
COVERAGE_PCT: 91.39%

Audit note: VERIFIED counts symbols reachable through either the default safe API or the public `raw-ffi` feature.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CGEventCreate | function | CGEvent.h | `raw_ffi::CGEventCreate`, `Event::new()` |
| CGEventCreateCopy | function | CGEvent.h | `raw_ffi::CGEventCreateCopy`, `Event::copy()` |
| CGEventCreateData | function | CGEvent.h | `raw_ffi::CGEventCreateData`, `Event::data()` |
| CGEventCreateFromData | function | CGEvent.h | `raw_ffi::CGEventCreateFromData`, `Event::from_data()` |
| CGEventCreateKeyboardEvent | function | CGEvent.h | `raw_ffi::CGEventCreateKeyboardEvent`, `KeyEvent` builders |
| CGEventCreateMouseEvent | function | CGEvent.h | `raw_ffi::CGEventCreateMouseEvent`, `MouseEvent` builders |
| CGEventCreateScrollWheelEvent | function | CGEvent.h | `raw_ffi::CGEventCreateScrollWheelEvent`, `ScrollEvent` builders |
| CGEventCreateScrollWheelEvent2 | function | CGEvent.h | `raw_ffi::CGEventCreateScrollWheelEvent2`, `ScrollEvent` builders |
| CGEventCreateSourceFromEvent | function | CGEvent.h | `raw_ffi::CGEventCreateSourceFromEvent`, `Event::source()` |
| CGEventGetDoubleValueField | function | CGEvent.h | `raw_ffi::CGEventGetDoubleValueField`, `Event::double_value()` |
| CGEventGetFlags | function | CGEvent.h | `raw_ffi::CGEventGetFlags`, `Event::flags()` |
| CGEventGetIntegerValueField | function | CGEvent.h | `raw_ffi::CGEventGetIntegerValueField`, `Event::integer_value()` |
| CGEventGetLocation | function | CGEvent.h | `raw_ffi::CGEventGetLocation`, `Event::location()` |
| CGEventGetTimestamp | function | CGEvent.h | `raw_ffi::CGEventGetTimestamp`, `Event::timestamp()` |
| CGEventGetType | function | CGEvent.h | `raw_ffi::CGEventGetType`, `Event::event_type()` |
| CGEventGetTypeID | function | CGEvent.h | `raw_ffi::CGEventGetTypeID`, `Event::type_id()` |
| CGEventGetUnflippedLocation | function | CGEvent.h | `raw_ffi::CGEventGetUnflippedLocation`, `Event::unflipped_location()` |
| CGEventKeyboardGetUnicodeString | function | CGEvent.h | `raw_ffi::CGEventKeyboardGetUnicodeString`, `Event::unicode_string()` |
| CGEventKeyboardSetUnicodeString | function | CGEvent.h | `raw_ffi::CGEventKeyboardSetUnicodeString`, `Event::set_unicode_string()` |
| CGEventPost | function | CGEvent.h | `raw_ffi::CGEventPost`, `Event::post()` |
| CGEventPostToPid | function | CGEvent.h | `raw_ffi::CGEventPostToPid`, `Event::post_to_pid()` |
| CGEventSetDoubleValueField | function | CGEvent.h | `raw_ffi::CGEventSetDoubleValueField`, `Event::set_double_value()` |
| CGEventSetFlags | function | CGEvent.h | `raw_ffi::CGEventSetFlags`, `Event::set_flags()` |
| CGEventSetIntegerValueField | function | CGEvent.h | `raw_ffi::CGEventSetIntegerValueField`, `Event::set_integer_value()` |
| CGEventSetLocation | function | CGEvent.h | `raw_ffi::CGEventSetLocation`, `Event::set_location()` |
| CGEventSetSource | function | CGEvent.h | `raw_ffi::CGEventSetSource`, `Event::set_source()` |
| CGEventSetTimestamp | function | CGEvent.h | `raw_ffi::CGEventSetTimestamp`, `Event::set_timestamp()` |
| CGEventSetType | function | CGEvent.h | `raw_ffi::CGEventSetType`, `Event::set_type()` |
| CGEventTapCreate | function | CGEvent.h | `raw_ffi::CGEventTapCreate`, `EventTap::new_with_options()` |
| CGEventTapCreateForPSN | function | CGEvent.h | `raw_ffi::CGEventTapCreateForPSN` |
| CGEventTapCreateForPid | function | CGEvent.h | `raw_ffi::CGEventTapCreateForPid`, `EventTap::for_pid()` |
| CGEventTapEnable | function | CGEvent.h | `raw_ffi::CGEventTapEnable`, `EventTap::enable()/disable()` |
| CGEventTapIsEnabled | function | CGEvent.h | `raw_ffi::CGEventTapIsEnabled`, `EventTap::is_enabled()` |
| CGEventTapPostEvent | function | CGEvent.h | `raw_ffi::CGEventTapPostEvent`, `CGEventTapProxy::post_event()` |
| CGGetEventTapList | function | CGEvent.h | `raw_ffi::CGGetEventTapList`, `EventTap::installed()` |
| CGPreflightListenEventAccess | function | CGEvent.h | `raw_ffi::CGPreflightListenEventAccess`, `EventTap::preflight_listen_access()` |
| CGPreflightPostEventAccess | function | CGEvent.h | `raw_ffi::CGPreflightPostEventAccess`, `EventTap::preflight_post_access()` |
| CGRequestListenEventAccess | function | CGEvent.h | `raw_ffi::CGRequestListenEventAccess`, `EventTap::request_listen_access()` |
| CGRequestPostEventAccess | function | CGEvent.h | `raw_ffi::CGRequestPostEventAccess`, `EventTap::request_post_access()` |
| CGEventSourceButtonState | function | CGEventSource.h | `raw_ffi::CGEventSourceButtonState`, `EventSource::button_state()` |
| CGEventSourceCounterForEventType | function | CGEventSource.h | `raw_ffi::CGEventSourceCounterForEventType`, `EventSource::counter_for_event_type()` |
| CGEventSourceCreate | function | CGEventSource.h | `raw_ffi::CGEventSourceCreate`, `EventSource::new()` |
| CGEventSourceFlagsState | function | CGEventSource.h | `raw_ffi::CGEventSourceFlagsState`, `EventSource::flags_state()` |
| CGEventSourceGetKeyboardType | function | CGEventSource.h | `raw_ffi::CGEventSourceGetKeyboardType`, `EventSource::keyboard_type()` |
| CGEventSourceGetLocalEventsFilterDuringSuppressionState | function | CGEventSource.h | `raw_ffi::CGEventSourceGetLocalEventsFilterDuringSuppressionState`, `EventSource::local_events_filter()` |
| CGEventSourceGetLocalEventsSuppressionInterval | function | CGEventSource.h | `raw_ffi::CGEventSourceGetLocalEventsSuppressionInterval`, `EventSource::local_events_suppression_interval()` |
| CGEventSourceGetPixelsPerLine | function | CGEventSource.h | `raw_ffi::CGEventSourceGetPixelsPerLine`, `EventSource::pixels_per_line()` |
| CGEventSourceGetSourceStateID | function | CGEventSource.h | `raw_ffi::CGEventSourceGetSourceStateID`, `EventSource::source_state_id()` |
| CGEventSourceGetTypeID | function | CGEventSource.h | `raw_ffi::CGEventSourceGetTypeID`, `EventSource::type_id()` |
| CGEventSourceGetUserData | function | CGEventSource.h | `raw_ffi::CGEventSourceGetUserData`, `EventSource::user_data()` |
| CGEventSourceKeyState | function | CGEventSource.h | `raw_ffi::CGEventSourceKeyState`, `EventSource::key_state()` |
| CGEventSourceSecondsSinceLastEventType | function | CGEventSource.h | `raw_ffi::CGEventSourceSecondsSinceLastEventType`, `EventSource::seconds_since_last_event_type()` |
| CGEventSourceSetKeyboardType | function | CGEventSource.h | `raw_ffi::CGEventSourceSetKeyboardType`, `EventSource::set_keyboard_type()` |
| CGEventSourceSetLocalEventsFilterDuringSuppressionState | function | CGEventSource.h | `raw_ffi::CGEventSourceSetLocalEventsFilterDuringSuppressionState`, `EventSource::set_local_events_filter()` |
| CGEventSourceSetLocalEventsSuppressionInterval | function | CGEventSource.h | `raw_ffi::CGEventSourceSetLocalEventsSuppressionInterval`, `EventSource::set_local_events_suppression_interval()` |
| CGEventSourceSetPixelsPerLine | function | CGEventSource.h | `raw_ffi::CGEventSourceSetPixelsPerLine`, `EventSource::set_pixels_per_line()` |
| CGEventSourceSetUserData | function | CGEventSource.h | `raw_ffi::CGEventSourceSetUserData`, `EventSource::set_user_data()` |
| CGEventField | enum | CGEventTypes.h | `raw_ffi::CGEventField`, `cgevents::CGEventField` |
| CGEventFlags | bitflags | CGEventTypes.h | `raw_ffi::CGEventFlags`, `cgevents::CGEventFlags` |
| CGEventMask | type | CGEventTypes.h | `raw_ffi::CGEventMask`, `u64` masks on `EventTap` |
| CGEventMaskBit | macro | CGEventTypes.h | `raw_ffi::cg_event_mask_bit()`, `CGEventType::mask_bit()` |
| CGEventMouseSubtype | enum | CGEventTypes.h | `raw_ffi::CGEventMouseSubtype`, `cgevents::CGEventMouseSubtype` |
| CGEventRef | type | CGEventTypes.h | `raw_ffi::CGEventRef`, `Event` |
| CGEventSourceKeyboardType | type | CGEventTypes.h | `raw_ffi::CGEventSourceKeyboardType`, `EventSource::keyboard_type()` |
| CGEventSourceRef | type | CGEventTypes.h | `raw_ffi::CGEventSourceRef`, `EventSource` |
| CGEventSourceStateID | enum | CGEventTypes.h | `raw_ffi::CGEventSourceStateID`, `SourceState` |
| CGEventTapCallBack | type | CGEventTypes.h | `raw_ffi::CGEventTapCallBack` |
| CGEventTapInformation | type | CGEventTypes.h | `raw_ffi::CGEventTapInformation`, `EventTapInformation` |
| CGEventTapLocation | enum | CGEventTypes.h | `raw_ffi::CGEventTapLocation`, `cgevents::CGEventTapLocation` |
| CGEventTapOptions | enum | CGEventTypes.h | `raw_ffi::CGEventTapOptions`, `cgevents::CGEventTapOptions` |
| CGEventTapPlacement | enum | CGEventTypes.h | `raw_ffi::CGEventTapPlacement`, `TapPlacement` |
| CGEventTapProxy | type | CGEventTypes.h | `raw_ffi::CGEventTapProxy`, `cgevents::CGEventTapProxy` |
| CGEventTimestamp | type | CGEventTypes.h | `raw_ffi::CGEventTimestamp`, `cgevents::CGEventTimestamp` |
| CGEventType | enum | CGEventTypes.h | `raw_ffi::CGEventType`, `cgevents::CGEventType` |
| CGMouseButton | enum | CGEventTypes.h | `raw_ffi::CGMouseButton`, `MouseButton` |
| CGScrollEventUnit | enum | CGEventTypes.h | `raw_ffi::CGScrollEventUnit`, `ScrollEvent` builders |
| kCGAnnotatedSessionEventTap | constant | CGEventTypes.h | `raw_ffi::kCGAnnotatedSessionEventTap` |
| kCGAnyInputEventType | constant | CGEventTypes.h | `raw_ffi::kCGAnyInputEventType`, `CG_ANY_INPUT_EVENT_TYPE` |
| kCGEventFlagMaskAlphaShift | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskAlphaShift` |
| kCGEventFlagMaskAlternate | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskAlternate` |
| kCGEventFlagMaskCommand | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskCommand` |
| kCGEventFlagMaskControl | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskControl` |
| kCGEventFlagMaskHelp | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskHelp` |
| kCGEventFlagMaskNonCoalesced | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskNonCoalesced` |
| kCGEventFlagMaskNumericPad | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskNumericPad` |
| kCGEventFlagMaskSecondaryFn | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskSecondaryFn` |
| kCGEventFlagMaskShift | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagMaskShift` |
| kCGEventFlagsChanged | constant | CGEventTypes.h | `raw_ffi::kCGEventFlagsChanged` |
| kCGEventKeyDown | constant | CGEventTypes.h | `raw_ffi::kCGEventKeyDown` |
| kCGEventKeyUp | constant | CGEventTypes.h | `raw_ffi::kCGEventKeyUp` |
| kCGEventLeftMouseDown | constant | CGEventTypes.h | `raw_ffi::kCGEventLeftMouseDown` |
| kCGEventLeftMouseDragged | constant | CGEventTypes.h | `raw_ffi::kCGEventLeftMouseDragged` |
| kCGEventLeftMouseUp | constant | CGEventTypes.h | `raw_ffi::kCGEventLeftMouseUp` |
| kCGEventMaskForAllEvents | constant | CGEventTypes.h | `raw_ffi::kCGEventMaskForAllEvents`, `CG_EVENT_MASK_FOR_ALL_EVENTS` |
| kCGEventMouseMoved | constant | CGEventTypes.h | `raw_ffi::kCGEventMouseMoved` |
| kCGEventMouseSubtypeDefault | constant | CGEventTypes.h | `raw_ffi::kCGEventMouseSubtypeDefault` |
| kCGEventMouseSubtypeTabletPoint | constant | CGEventTypes.h | `raw_ffi::kCGEventMouseSubtypeTabletPoint` |
| kCGEventMouseSubtypeTabletProximity | constant | CGEventTypes.h | `raw_ffi::kCGEventMouseSubtypeTabletProximity` |
| kCGEventNull | constant | CGEventTypes.h | `raw_ffi::kCGEventNull` |
| kCGEventOtherMouseDown | constant | CGEventTypes.h | `raw_ffi::kCGEventOtherMouseDown` |
| kCGEventOtherMouseDragged | constant | CGEventTypes.h | `raw_ffi::kCGEventOtherMouseDragged` |
| kCGEventOtherMouseUp | constant | CGEventTypes.h | `raw_ffi::kCGEventOtherMouseUp` |
| kCGEventRightMouseDown | constant | CGEventTypes.h | `raw_ffi::kCGEventRightMouseDown` |
| kCGEventRightMouseDragged | constant | CGEventTypes.h | `raw_ffi::kCGEventRightMouseDragged` |
| kCGEventRightMouseUp | constant | CGEventTypes.h | `raw_ffi::kCGEventRightMouseUp` |
| kCGEventScrollWheel | constant | CGEventTypes.h | `raw_ffi::kCGEventScrollWheel` |
| kCGEventSourceGroupID | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceGroupID` |
| kCGEventSourceStateCombinedSessionState | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceStateCombinedSessionState` |
| kCGEventSourceStateHIDSystemState | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceStateHIDSystemState` |
| kCGEventSourceStateID | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceStateID` |
| kCGEventSourceStatePrivate | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceStatePrivate` |
| kCGEventSourceUnixProcessID | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceUnixProcessID` |
| kCGEventSourceUserData | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceUserData` |
| kCGEventSourceUserID | constant | CGEventTypes.h | `raw_ffi::kCGEventSourceUserID` |
| kCGEventTabletPointer | constant | CGEventTypes.h | `raw_ffi::kCGEventTabletPointer` |
| kCGEventTabletProximity | constant | CGEventTypes.h | `raw_ffi::kCGEventTabletProximity` |
| kCGEventTapDisabledByTimeout | constant | CGEventTypes.h | `raw_ffi::kCGEventTapDisabledByTimeout` |
| kCGEventTapDisabledByUserInput | constant | CGEventTypes.h | `raw_ffi::kCGEventTapDisabledByUserInput` |
| kCGEventTapOptionDefault | constant | CGEventTypes.h | `raw_ffi::kCGEventTapOptionDefault` |
| kCGEventTapOptionListenOnly | constant | CGEventTypes.h | `raw_ffi::kCGEventTapOptionListenOnly` |
| kCGEventTargetProcessSerialNumber | constant | CGEventTypes.h | `raw_ffi::kCGEventTargetProcessSerialNumber` |
| kCGEventTargetUnixProcessID | constant | CGEventTypes.h | `raw_ffi::kCGEventTargetUnixProcessID` |
| kCGEventUnacceleratedPointerMovementX | constant | CGEventTypes.h | `raw_ffi::kCGEventUnacceleratedPointerMovementX` |
| kCGEventUnacceleratedPointerMovementY | constant | CGEventTypes.h | `raw_ffi::kCGEventUnacceleratedPointerMovementY` |
| kCGHIDEventTap | constant | CGEventTypes.h | `raw_ffi::kCGHIDEventTap` |
| kCGHeadInsertEventTap | constant | CGEventTypes.h | `raw_ffi::kCGHeadInsertEventTap` |
| kCGKeyboardEventAutorepeat | constant | CGEventTypes.h | `raw_ffi::kCGKeyboardEventAutorepeat` |
| kCGKeyboardEventKeyboardType | constant | CGEventTypes.h | `raw_ffi::kCGKeyboardEventKeyboardType` |
| kCGKeyboardEventKeycode | constant | CGEventTypes.h | `raw_ffi::kCGKeyboardEventKeycode` |
| kCGMouseButtonCenter | constant | CGEventTypes.h | `raw_ffi::kCGMouseButtonCenter` |
| kCGMouseButtonLeft | constant | CGEventTypes.h | `raw_ffi::kCGMouseButtonLeft` |
| kCGMouseButtonRight | constant | CGEventTypes.h | `raw_ffi::kCGMouseButtonRight` |
| kCGMouseEventButtonNumber | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventButtonNumber` |
| kCGMouseEventClickState | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventClickState` |
| kCGMouseEventDeltaX | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventDeltaX` |
| kCGMouseEventDeltaY | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventDeltaY` |
| kCGMouseEventInstantMouser | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventInstantMouser` |
| kCGMouseEventNumber | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventNumber` |
| kCGMouseEventPressure | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventPressure` |
| kCGMouseEventSubtype | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventSubtype` |
| kCGMouseEventWindowUnderMousePointer | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventWindowUnderMousePointer` |
| kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent | constant | CGEventTypes.h | `raw_ffi::kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent` |
| kCGNotifyEventTapAdded | constant | CGEventTypes.h | `raw_ffi::kCGNotifyEventTapAdded`, `EVENT_TAP_ADDED_NOTIFICATION` |
| kCGNotifyEventTapRemoved | constant | CGEventTypes.h | `raw_ffi::kCGNotifyEventTapRemoved`, `EVENT_TAP_REMOVED_NOTIFICATION` |
| kCGScrollEventUnitLine | constant | CGEventTypes.h | `raw_ffi::kCGScrollEventUnitLine` |
| kCGScrollEventUnitPixel | constant | CGEventTypes.h | `raw_ffi::kCGScrollEventUnitPixel` |
| kCGScrollWheelEventAcceleratedDeltaAxis1 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventAcceleratedDeltaAxis1` |
| kCGScrollWheelEventAcceleratedDeltaAxis2 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventAcceleratedDeltaAxis2` |
| kCGScrollWheelEventDeltaAxis1 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventDeltaAxis1` |
| kCGScrollWheelEventDeltaAxis2 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventDeltaAxis2` |
| kCGScrollWheelEventDeltaAxis3 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventDeltaAxis3` |
| kCGScrollWheelEventFixedPtDeltaAxis1 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventFixedPtDeltaAxis1` |
| kCGScrollWheelEventFixedPtDeltaAxis2 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventFixedPtDeltaAxis2` |
| kCGScrollWheelEventFixedPtDeltaAxis3 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventFixedPtDeltaAxis3` |
| kCGScrollWheelEventInstantMouser | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventInstantMouser` |
| kCGScrollWheelEventIsContinuous | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventIsContinuous` |
| kCGScrollWheelEventMomentumOptionPhase | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventMomentumOptionPhase` |
| kCGScrollWheelEventMomentumPhase | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventMomentumPhase` |
| kCGScrollWheelEventPointDeltaAxis1 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventPointDeltaAxis1` |
| kCGScrollWheelEventPointDeltaAxis2 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventPointDeltaAxis2` |
| kCGScrollWheelEventPointDeltaAxis3 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventPointDeltaAxis3` |
| kCGScrollWheelEventRawDeltaAxis1 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventRawDeltaAxis1` |
| kCGScrollWheelEventRawDeltaAxis2 | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventRawDeltaAxis2` |
| kCGScrollWheelEventScrollCount | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventScrollCount` |
| kCGScrollWheelEventScrollPhase | constant | CGEventTypes.h | `raw_ffi::kCGScrollWheelEventScrollPhase` |
| kCGSessionEventTap | constant | CGEventTypes.h | `raw_ffi::kCGSessionEventTap` |
| kCGTabletEventDeviceID | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventDeviceID` |
| kCGTabletEventPointButtons | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventPointButtons` |
| kCGTabletEventPointPressure | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventPointPressure` |
| kCGTabletEventPointX | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventPointX` |
| kCGTabletEventPointY | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventPointY` |
| kCGTabletEventPointZ | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventPointZ` |
| kCGTabletEventRotation | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventRotation` |
| kCGTabletEventTangentialPressure | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventTangentialPressure` |
| kCGTabletEventTiltX | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventTiltX` |
| kCGTabletEventTiltY | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventTiltY` |
| kCGTabletEventVendor1 | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventVendor1` |
| kCGTabletEventVendor2 | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventVendor2` |
| kCGTabletEventVendor3 | constant | CGEventTypes.h | `raw_ffi::kCGTabletEventVendor3` |
| kCGTabletProximityEventCapabilityMask | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventCapabilityMask` |
| kCGTabletProximityEventDeviceID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventDeviceID` |
| kCGTabletProximityEventEnterProximity | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventEnterProximity` |
| kCGTabletProximityEventPointerID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventPointerID` |
| kCGTabletProximityEventPointerType | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventPointerType` |
| kCGTabletProximityEventSystemTabletID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventSystemTabletID` |
| kCGTabletProximityEventTabletID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventTabletID` |
| kCGTabletProximityEventVendorID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventVendorID` |
| kCGTabletProximityEventVendorPointerSerialNumber | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventVendorPointerSerialNumber` |
| kCGTabletProximityEventVendorPointerType | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventVendorPointerType` |
| kCGTabletProximityEventVendorUniqueID | constant | CGEventTypes.h | `raw_ffi::kCGTabletProximityEventVendorUniqueID` |
| kCGTailAppendEventTap | constant | CGEventTypes.h | `raw_ffi::kCGTailAppendEventTap` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CGGesturePhase | enum | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| CGMomentumScrollPhase | enum | CGEventTypes.h | Missing named momentum-phase representation; callers must interpret `CGEventField::ScrollWheelEventMomentumPhase` numerically. |
| CGScrollPhase | enum | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |
| kCGGesturePhaseBegan | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGGesturePhaseCancelled | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGGesturePhaseChanged | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGGesturePhaseEnded | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGGesturePhaseMayBegin | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGGesturePhaseNone | constant | CGEventTypes.h | Missing named gesture-phase representation; the crate exposes no raw or safe `CGGesturePhase` constants. |
| kCGMomentumScrollPhaseBegin | constant | CGEventTypes.h | Missing named momentum-phase representation; callers must interpret `CGEventField::ScrollWheelEventMomentumPhase` numerically. |
| kCGMomentumScrollPhaseContinue | constant | CGEventTypes.h | Missing named momentum-phase representation; callers must interpret `CGEventField::ScrollWheelEventMomentumPhase` numerically. |
| kCGMomentumScrollPhaseEnd | constant | CGEventTypes.h | Missing named momentum-phase representation; callers must interpret `CGEventField::ScrollWheelEventMomentumPhase` numerically. |
| kCGMomentumScrollPhaseNone | constant | CGEventTypes.h | Missing named momentum-phase representation; callers must interpret `CGEventField::ScrollWheelEventMomentumPhase` numerically. |
| kCGScrollPhaseBegan | constant | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |
| kCGScrollPhaseCancelled | constant | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |
| kCGScrollPhaseChanged | constant | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |
| kCGScrollPhaseEnded | constant | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |
| kCGScrollPhaseMayBegin | constant | CGEventTypes.h | Missing named scroll-phase representation; callers must interpret `CGEventField::ScrollWheelEventScrollPhase` numerically. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CGEventPostToPSN | function | CGEvent.h | Legacy Process Serial Number posting API is excluded from scoring; `raw_ffi::CGEventPostToPSN` still exists for low-level callers. | `/* DEPRECATED; use CGEventPostToPid instead. */` |

## Notes

- The only uncovered named symbols are the three phase enums introduced in `CGEventTypes.h` (`CGMomentumScrollPhase`, `CGScrollPhase`, `CGGesturePhase`) and their constants.
- `cgevents` can still read/write the underlying scroll-phase and momentum-phase fields numerically via `CGEventField`, but it does not currently export the named phase enums/constants.
- `CGEventPostToPSN` is intentionally scored as exempt because the header marks it deprecated; the raw FFI still exposes it for completeness.
