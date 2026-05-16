# Quartz Event Services coverage audit

Crate: `cgevents`  
Target release: `0.5.1`

Reference headers audited:

- `CoreGraphics.framework/Headers/CGEvent.h`
- `CoreGraphics.framework/Headers/CGEventSource.h`
- `CoreGraphics.framework/Headers/CGEventTypes.h`

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped — entitlement-only / iOS-only / OS X 10.x deprecated

## Functions — `CGEvent.h`

| API | Status | Notes |
| --- | --- | --- |
| `CGEventGetTypeID` | ✅ implemented | `Event::type_id()` + Swift bridge `cgevent_get_type_id`. |
| `CGEventCreate` | ✅ implemented | `Event::new`. |
| `CGEventCreateData` | ✅ implemented | `Event::data()` on the Swift bridge (macOS 12+) and raw C symbol under `raw-ffi`. |
| `CGEventCreateFromData` | ✅ implemented | `Event::from_data`. |
| `CGEventCreateMouseEvent` | ✅ implemented | `MouseEvent::build`. |
| `CGEventCreateKeyboardEvent` | ✅ implemented | `KeyEvent::build`. |
| `CGEventCreateScrollWheelEvent` | ✅ implemented | `ScrollEvent::build` (1-axis path). |
| `CGEventCreateScrollWheelEvent2` | ✅ implemented | `ScrollEvent::build` (2D / 3D path). |
| `CGEventCreateCopy` | ✅ implemented | `Event::copy`. |
| `CGEventCreateSourceFromEvent` | ✅ implemented | `Event::source`. |
| `CGEventSetSource` | ✅ implemented | `Event::set_source`. |
| `CGEventGetType` | ✅ implemented | `Event::event_type` / `event_type_typed`. |
| `CGEventSetType` | ✅ implemented | `Event::set_type` / `set_event_type`. |
| `CGEventGetTimestamp` | ✅ implemented | `Event::timestamp` / `event_timestamp`. |
| `CGEventSetTimestamp` | ✅ implemented | `Event::set_timestamp` / `set_event_timestamp`. |
| `CGEventGetLocation` | ✅ implemented | `Event::location`, `TappedEvent::location`. |
| `CGEventGetUnflippedLocation` | ✅ implemented | `Event::unflipped_location`. |
| `CGEventSetLocation` | ✅ implemented | `Event::set_location`, `TappedEvent::set_location`. |
| `CGEventGetFlags` | ✅ implemented | `Event::flags`, `TappedEvent::flags`. |
| `CGEventSetFlags` | ✅ implemented | `Event::set_flags`, `TappedEvent::set_flags`. |
| `CGEventKeyboardGetUnicodeString` | ✅ implemented | `Event::unicode_string`, `TappedEvent::unicode_string`. |
| `CGEventKeyboardSetUnicodeString` | ✅ implemented | `Event::set_unicode_string`, `TappedEvent::set_unicode_string`. |
| `CGEventGetIntegerValueField` | ✅ implemented | `Event::integer_field` / `integer_value`, `TappedEvent::integer_value`. |
| `CGEventSetIntegerValueField` | ✅ implemented | `Event::set_integer_field` / `set_integer_value`, `TappedEvent::set_integer_value`. |
| `CGEventGetDoubleValueField` | ✅ implemented | `Event::double_field` / `double_value`, `TappedEvent::double_value`. |
| `CGEventSetDoubleValueField` | ✅ implemented | `Event::set_double_field` / `set_double_value`, `TappedEvent::set_double_value`. |
| `CGEventTapCreate` | ✅ implemented | `EventTap::new`, `EventTap::new_with_options`. |
| `CGEventTapCreateForPSN` | ⏭️ skipped — OS X 10.x deprecated | Safe wrapper intentionally omitted; legacy raw C symbol remains under `raw-ffi`. |
| `CGEventTapCreateForPid` | ✅ implemented | `EventTap::for_pid`. |
| `CGEventTapEnable` | ✅ implemented | `EventTap::enable`, `EventTap::disable`. |
| `CGEventTapIsEnabled` | ✅ implemented | `EventTap::is_enabled`. |
| `CGEventTapPostEvent` | ✅ implemented | `CGEventTapProxy::post_event`, `TappedEvent::post`. |
| `CGEventPost` | ✅ implemented | `Event::post`, builder `post` helpers. |
| `CGEventPostToPSN` | ⏭️ skipped — OS X 10.x deprecated | Safe wrapper intentionally omitted; legacy raw C symbol remains under `raw-ffi`. |
| `CGEventPostToPid` | ✅ implemented | `Event::post_to_pid`, builder `post_to_pid` helpers. |
| `CGGetEventTapList` | ✅ implemented | `EventTap::installed`. |
| `CGPreflightListenEventAccess` | ✅ implemented | `EventTap::preflight_listen_access`. |
| `CGRequestListenEventAccess` | ✅ implemented | `EventTap::request_listen_access`. |
| `CGPreflightPostEventAccess` | ✅ implemented | `EventTap::preflight_post_access`. |
| `CGRequestPostEventAccess` | ✅ implemented | `EventTap::request_post_access`. |

## Functions — `CGEventSource.h`

| API | Status | Notes |
| --- | --- | --- |
| `CGEventSourceGetTypeID` | ✅ implemented | `EventSource::type_id`. |
| `CGEventSourceCreate` | ✅ implemented | `EventSource::new`, `EventSource::private`. |
| `CGEventSourceGetKeyboardType` | ✅ implemented | `EventSource::keyboard_type`. |
| `CGEventSourceSetKeyboardType` | ✅ implemented | `EventSource::set_keyboard_type`. |
| `CGEventSourceGetPixelsPerLine` | ✅ implemented | `EventSource::pixels_per_line`. |
| `CGEventSourceSetPixelsPerLine` | ✅ implemented | `EventSource::set_pixels_per_line`. |
| `CGEventSourceGetSourceStateID` | ✅ implemented | `EventSource::source_state_id`. |
| `CGEventSourceButtonState` | ✅ implemented | `EventSource::button_state`. |
| `CGEventSourceKeyState` | ✅ implemented | `EventSource::key_state`. |
| `CGEventSourceFlagsState` | ✅ implemented | `EventSource::flags_state`. |
| `CGEventSourceSecondsSinceLastEventType` | ✅ implemented | `EventSource::seconds_since_last_event_type`, `seconds_since_last_typed_event`, `seconds_since_last_input_event`. |
| `CGEventSourceCounterForEventType` | ✅ implemented | `EventSource::counter_for_event_type`, `counter_for_typed_event`. |
| `CGEventSourceSetUserData` | ✅ implemented | `EventSource::set_user_data`. |
| `CGEventSourceGetUserData` | ✅ implemented | `EventSource::user_data`. |
| `CGEventSourceSetLocalEventsFilterDuringSuppressionState` | ✅ implemented | `EventSource::set_local_events_filter`. |
| `CGEventSourceGetLocalEventsFilterDuringSuppressionState` | ✅ implemented | `EventSource::local_events_filter`. |
| `CGEventSourceSetLocalEventsSuppressionInterval` | ✅ implemented | `EventSource::set_local_events_suppression_interval`. |
| `CGEventSourceGetLocalEventsSuppressionInterval` | ✅ implemented | `EventSource::local_events_suppression_interval`. |

## Logical area coverage — `CGEvent`

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGEventRef` | ✅ implemented | `Event` owned handle. |
| `CGEventGetTypeID` | ✅ implemented | `Event::type_id`. |
| `CGEventCreate*` family | ✅ implemented | `Event`, `KeyEvent`, `MouseEvent`, `ScrollEvent`. |
| `CGEvent` copy / source / post helpers | ✅ implemented | `Event::{copy,source,set_source,post,post_to_pid}`. |
| `CGEvent` field/timestamp/location/flags accessors | ✅ implemented | `Event` + `TappedEvent` accessors/setters. |
| `CGEvent` unicode helpers | ✅ implemented | `Event::{unicode_string,set_unicode_string}` + `type_string`. |

## Logical area coverage — `CGEventSource`

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGEventSourceRef` | ✅ implemented | `EventSource` owned handle. |
| `CGEventSourceStateID` | ✅ implemented | `SourceState` + `EventSource::source_state_id`. |
| `CGEventSourceKeyboardType` | ✅ implemented | `u32` keyboard type getters/setters. |
| Source state / counters / user data / suppression APIs | ✅ implemented | `EventSource` methods listed above. |

## Logical area coverage — `CGEventTap`

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGEventTapCreate` | ✅ implemented | `EventTap::{new,new_with_options}`. |
| `CGEventTapCreateForPid` | ✅ implemented | `EventTap::for_pid`. |
| `CGEventTapEnable` / `CGEventTapIsEnabled` | ✅ implemented | `EventTap::{enable,disable,is_enabled}`. |
| `CGGetEventTapList` | ✅ implemented | `EventTap::installed`. |
| `CGPreflight*` / `CGRequest*` access helpers | ✅ implemented | `EventTap::{preflight,request}_{listen,post}_access`. |
| `CGEventTapPlacement` | ✅ implemented | `TapPlacement`. |
| `CGEventTapInformation` | ✅ implemented | `EventTapInformation`. |
| `kCGNotifyEventTapAdded` / `kCGNotifyEventTapRemoved` | ✅ implemented | `EVENT_TAP_ADDED_NOTIFICATION`, `EVENT_TAP_REMOVED_NOTIFICATION`. |

## Logical area coverage — `CGEventField`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGMouseEventNumber` | ✅ implemented | `CGEventField::MouseEventNumber` |
| `kCGMouseEventClickState` | ✅ implemented | `CGEventField::MouseEventClickState` |
| `kCGMouseEventPressure` | ✅ implemented | `CGEventField::MouseEventPressure` |
| `kCGMouseEventButtonNumber` | ✅ implemented | `CGEventField::MouseEventButtonNumber` |
| `kCGMouseEventDeltaX` | ✅ implemented | `CGEventField::MouseEventDeltaX` |
| `kCGMouseEventDeltaY` | ✅ implemented | `CGEventField::MouseEventDeltaY` |
| `kCGMouseEventInstantMouser` | ✅ implemented | `CGEventField::MouseEventInstantMouser` |
| `kCGMouseEventSubtype` | ✅ implemented | `CGEventField::MouseEventSubtype` |
| `kCGKeyboardEventAutorepeat` | ✅ implemented | `CGEventField::KeyboardEventAutorepeat` |
| `kCGKeyboardEventKeycode` | ✅ implemented | `CGEventField::KeyboardEventKeycode` |
| `kCGKeyboardEventKeyboardType` | ✅ implemented | `CGEventField::KeyboardEventKeyboardType` |
| `kCGScrollWheelEventDeltaAxis1` | ✅ implemented | `CGEventField::ScrollWheelEventDeltaAxis1` |
| `kCGScrollWheelEventDeltaAxis2` | ✅ implemented | `CGEventField::ScrollWheelEventDeltaAxis2` |
| `kCGScrollWheelEventDeltaAxis3` | ✅ implemented | `CGEventField::ScrollWheelEventDeltaAxis3` |
| `kCGScrollWheelEventFixedPtDeltaAxis1` | ✅ implemented | `CGEventField::ScrollWheelEventFixedPtDeltaAxis1` |
| `kCGScrollWheelEventFixedPtDeltaAxis2` | ✅ implemented | `CGEventField::ScrollWheelEventFixedPtDeltaAxis2` |
| `kCGScrollWheelEventFixedPtDeltaAxis3` | ✅ implemented | `CGEventField::ScrollWheelEventFixedPtDeltaAxis3` |
| `kCGScrollWheelEventPointDeltaAxis1` | ✅ implemented | `CGEventField::ScrollWheelEventPointDeltaAxis1` |
| `kCGScrollWheelEventPointDeltaAxis2` | ✅ implemented | `CGEventField::ScrollWheelEventPointDeltaAxis2` |
| `kCGScrollWheelEventPointDeltaAxis3` | ✅ implemented | `CGEventField::ScrollWheelEventPointDeltaAxis3` |
| `kCGScrollWheelEventScrollPhase` | ✅ implemented | `CGEventField::ScrollWheelEventScrollPhase` |
| `kCGScrollWheelEventScrollCount` | ✅ implemented | `CGEventField::ScrollWheelEventScrollCount` |
| `kCGScrollWheelEventMomentumPhase` | ✅ implemented | `CGEventField::ScrollWheelEventMomentumPhase` |
| `kCGScrollWheelEventInstantMouser` | ✅ implemented | `CGEventField::ScrollWheelEventInstantMouser` |
| `kCGTabletEventPointX` | ✅ implemented | `CGEventField::TabletEventPointX` |
| `kCGTabletEventPointY` | ✅ implemented | `CGEventField::TabletEventPointY` |
| `kCGTabletEventPointZ` | ✅ implemented | `CGEventField::TabletEventPointZ` |
| `kCGTabletEventPointButtons` | ✅ implemented | `CGEventField::TabletEventPointButtons` |
| `kCGTabletEventPointPressure` | ✅ implemented | `CGEventField::TabletEventPointPressure` |
| `kCGTabletEventTiltX` | ✅ implemented | `CGEventField::TabletEventTiltX` |
| `kCGTabletEventTiltY` | ✅ implemented | `CGEventField::TabletEventTiltY` |
| `kCGTabletEventRotation` | ✅ implemented | `CGEventField::TabletEventRotation` |
| `kCGTabletEventTangentialPressure` | ✅ implemented | `CGEventField::TabletEventTangentialPressure` |
| `kCGTabletEventDeviceID` | ✅ implemented | `CGEventField::TabletEventDeviceID` |
| `kCGTabletEventVendor1` | ✅ implemented | `CGEventField::TabletEventVendor1` |
| `kCGTabletEventVendor2` | ✅ implemented | `CGEventField::TabletEventVendor2` |
| `kCGTabletEventVendor3` | ✅ implemented | `CGEventField::TabletEventVendor3` |
| `kCGTabletProximityEventVendorID` | ✅ implemented | `CGEventField::TabletProximityEventVendorID` |
| `kCGTabletProximityEventTabletID` | ✅ implemented | `CGEventField::TabletProximityEventTabletID` |
| `kCGTabletProximityEventPointerID` | ✅ implemented | `CGEventField::TabletProximityEventPointerID` |
| `kCGTabletProximityEventDeviceID` | ✅ implemented | `CGEventField::TabletProximityEventDeviceID` |
| `kCGTabletProximityEventSystemTabletID` | ✅ implemented | `CGEventField::TabletProximityEventSystemTabletID` |
| `kCGTabletProximityEventVendorPointerType` | ✅ implemented | `CGEventField::TabletProximityEventVendorPointerType` |
| `kCGTabletProximityEventVendorPointerSerialNumber` | ✅ implemented | `CGEventField::TabletProximityEventVendorPointerSerialNumber` |
| `kCGTabletProximityEventVendorUniqueID` | ✅ implemented | `CGEventField::TabletProximityEventVendorUniqueID` |
| `kCGTabletProximityEventCapabilityMask` | ✅ implemented | `CGEventField::TabletProximityEventCapabilityMask` |
| `kCGTabletProximityEventPointerType` | ✅ implemented | `CGEventField::TabletProximityEventPointerType` |
| `kCGTabletProximityEventEnterProximity` | ✅ implemented | `CGEventField::TabletProximityEventEnterProximity` |
| `kCGEventTargetProcessSerialNumber` | ✅ implemented | `CGEventField::EventTargetProcessSerialNumber` |
| `kCGEventTargetUnixProcessID` | ✅ implemented | `CGEventField::EventTargetUnixProcessID` |
| `kCGEventSourceUnixProcessID` | ✅ implemented | `CGEventField::EventSourceUnixProcessID` |
| `kCGEventSourceUserData` | ✅ implemented | `CGEventField::EventSourceUserData` |
| `kCGEventSourceUserID` | ✅ implemented | `CGEventField::EventSourceUserID` |
| `kCGEventSourceGroupID` | ✅ implemented | `CGEventField::EventSourceGroupID` |
| `kCGEventSourceStateID` | ✅ implemented | `CGEventField::EventSourceStateID` |
| `kCGScrollWheelEventIsContinuous` | ✅ implemented | `CGEventField::ScrollWheelEventIsContinuous` |
| `kCGMouseEventWindowUnderMousePointer` | ✅ implemented | `CGEventField::MouseEventWindowUnderMousePointer` |
| `kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent` | ✅ implemented | `CGEventField::MouseEventWindowUnderMousePointerThatCanHandleThisEvent` |
| `kCGEventUnacceleratedPointerMovementX` | ✅ implemented | `CGEventField::EventUnacceleratedPointerMovementX` |
| `kCGEventUnacceleratedPointerMovementY` | ✅ implemented | `CGEventField::EventUnacceleratedPointerMovementY` |
| `kCGScrollWheelEventMomentumOptionPhase` | ✅ implemented | `CGEventField::ScrollWheelEventMomentumOptionPhase` |
| `kCGScrollWheelEventAcceleratedDeltaAxis1` | ✅ implemented | `CGEventField::ScrollWheelEventAcceleratedDeltaAxis1` |
| `kCGScrollWheelEventAcceleratedDeltaAxis2` | ✅ implemented | `CGEventField::ScrollWheelEventAcceleratedDeltaAxis2` |
| `kCGScrollWheelEventRawDeltaAxis1` | ✅ implemented | `CGEventField::ScrollWheelEventRawDeltaAxis1` |
| `kCGScrollWheelEventRawDeltaAxis2` | ✅ implemented | `CGEventField::ScrollWheelEventRawDeltaAxis2` |

## Logical area coverage — `CGEventType`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGEventNull` | ✅ implemented | `CGEventType::Null` |
| `kCGEventLeftMouseDown` | ✅ implemented | `CGEventType::LeftMouseDown` |
| `kCGEventLeftMouseUp` | ✅ implemented | `CGEventType::LeftMouseUp` |
| `kCGEventRightMouseDown` | ✅ implemented | `CGEventType::RightMouseDown` |
| `kCGEventRightMouseUp` | ✅ implemented | `CGEventType::RightMouseUp` |
| `kCGEventMouseMoved` | ✅ implemented | `CGEventType::MouseMoved` |
| `kCGEventLeftMouseDragged` | ✅ implemented | `CGEventType::LeftMouseDragged` |
| `kCGEventRightMouseDragged` | ✅ implemented | `CGEventType::RightMouseDragged` |
| `kCGEventKeyDown` | ✅ implemented | `CGEventType::KeyDown` |
| `kCGEventKeyUp` | ✅ implemented | `CGEventType::KeyUp` |
| `kCGEventFlagsChanged` | ✅ implemented | `CGEventType::FlagsChanged` |
| `kCGEventScrollWheel` | ✅ implemented | `CGEventType::ScrollWheel` |
| `kCGEventTabletPointer` | ✅ implemented | `CGEventType::TabletPointer` |
| `kCGEventTabletProximity` | ✅ implemented | `CGEventType::TabletProximity` |
| `kCGEventOtherMouseDown` | ✅ implemented | `CGEventType::OtherMouseDown` |
| `kCGEventOtherMouseUp` | ✅ implemented | `CGEventType::OtherMouseUp` |
| `kCGEventOtherMouseDragged` | ✅ implemented | `CGEventType::OtherMouseDragged` |
| `kCGEventTapDisabledByTimeout` | ✅ implemented | `CGEventType::TapDisabledByTimeout` |
| `kCGEventTapDisabledByUserInput` | ✅ implemented | `CGEventType::TapDisabledByUserInput` |
| `CGEventMaskBit(eventType)` | ✅ implemented | `CGEventType::mask_bit()` + raw helper `raw_ffi::cg_event_mask_bit`. |
| `kCGEventMaskForAllEvents` | ✅ implemented | `CG_EVENT_MASK_FOR_ALL_EVENTS`. |
| `kCGAnyInputEventType` | ✅ implemented | `CG_ANY_INPUT_EVENT_TYPE`. |

## Logical area coverage — `CGEventFlags`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGEventFlagMaskAlphaShift` | ✅ implemented | `CGEventFlags::ALPHA_SHIFT` |
| `kCGEventFlagMaskShift` | ✅ implemented | `CGEventFlags::SHIFT` |
| `kCGEventFlagMaskControl` | ✅ implemented | `CGEventFlags::CONTROL` |
| `kCGEventFlagMaskAlternate` | ✅ implemented | `CGEventFlags::ALTERNATE` |
| `kCGEventFlagMaskCommand` | ✅ implemented | `CGEventFlags::COMMAND` |
| `kCGEventFlagMaskHelp` | ✅ implemented | `CGEventFlags::HELP` |
| `kCGEventFlagMaskSecondaryFn` | ✅ implemented | `CGEventFlags::SECONDARY_FN` |
| `kCGEventFlagMaskNumericPad` | ✅ implemented | `CGEventFlags::NUMERIC_PAD` |
| `kCGEventFlagMaskNonCoalesced` | ✅ implemented | `CGEventFlags::NON_COALESCED` |

## Logical area coverage — `CGEventMouseSubtype`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGEventMouseSubtypeDefault` | ✅ implemented | `CGEventMouseSubtype::Default` |
| `kCGEventMouseSubtypeTabletPoint` | ✅ implemented | `CGEventMouseSubtype::TabletPoint` |
| `kCGEventMouseSubtypeTabletProximity` | ✅ implemented | `CGEventMouseSubtype::TabletProximity` |

## Logical area coverage — `CGEventTapLocation`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGHIDEventTap` | ✅ implemented | `CGEventTapLocation::Hid` |
| `kCGSessionEventTap` | ✅ implemented | `CGEventTapLocation::Session` |
| `kCGAnnotatedSessionEventTap` | ✅ implemented | `CGEventTapLocation::AnnotatedSession` |

## Logical area coverage — `CGEventTapOptions`

| Apple constant | Status | Rust surface |
| --- | --- | --- |
| `kCGEventTapOptionDefault` | ✅ implemented | `CGEventTapOptions::Default` |
| `kCGEventTapOptionListenOnly` | ✅ implemented | `CGEventTapOptions::ListenOnly` |

## Logical area coverage — `CGEventTapProxy`

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGEventTapProxy` | ✅ implemented | `CGEventTapProxy<'_>` callback wrapper. |
| `CGEventTapPostEvent` proxy usage | ✅ implemented | `CGEventTapProxy::post_event`, `TappedEvent::post`. |

## Logical area coverage — `CGEventTimestamp`

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGEventTimestamp` | ✅ implemented | `CGEventTimestamp(u64)` newtype. |
| `CGEventGetTimestamp` / `CGEventSetTimestamp` | ✅ implemented | `Event` + `TappedEvent` timestamp helpers. |

## Logical area coverage — phase enums

| Symbol | Status | Rust surface |
| --- | --- | --- |
| `CGGesturePhase` | ✅ implemented | `CGGesturePhase` safe enum + `raw_ffi` constants. |
| `CGMomentumScrollPhase` | ✅ implemented | `CGMomentumScrollPhase`, `Event::{momentum_scroll_phase,set_momentum_scroll_phase}`, `TappedEvent::{momentum_scroll_phase,set_momentum_scroll_phase}`. |
| `CGScrollPhase` | ✅ implemented | `CGScrollPhase`, `Event::{scroll_phase,set_scroll_phase}`, `TappedEvent::{scroll_phase,set_scroll_phase}`. |

## Related event-system support types (implemented)

- `CGMouseButton` → `MouseButton`
- `CGScrollEventUnit` → `ScrollEvent::{lines,pixels,...}`
- `CGEventTapPlacement` → `TapPlacement`
- `CGEventMask` → `u64` masks on `EventTap`
- `CGEventSourceStateID` → `SourceState` + raw `source_state_id()`
- `CGEventFilterMask` → `LocalEventsFilter`
- `CGEventSuppressionState` → `SuppressionState`
- `CGKeyCode` / `CGCharCode` / `CGButtonCount` / `CGWheelCount` → raw primitive aliases in `raw_ffi`, typed builders in safe API

## Deferred / intentionally skipped

1. `CGEventTapCreateForPSN` — deprecated in the 10.x era; omitted from the safe surface.
2. `CGEventPostToPSN` — deprecated in the 10.x era; omitted from the safe surface.

Both symbols remain available to low-level callers through `cgevents::raw_ffi` when the `raw-ffi` feature is enabled.
