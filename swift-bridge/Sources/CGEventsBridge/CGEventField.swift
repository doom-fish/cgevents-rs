import CoreGraphics

@_cdecl("cgevent_field_raw_value")
public func cgeventFieldRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGEventField.mouseEventNumber.rawValue
    case 1: return CGEventField.mouseEventClickState.rawValue
    case 2: return CGEventField.mouseEventPressure.rawValue
    case 3: return CGEventField.mouseEventButtonNumber.rawValue
    case 4: return CGEventField.mouseEventDeltaX.rawValue
    case 5: return CGEventField.mouseEventDeltaY.rawValue
    case 6: return CGEventField.mouseEventInstantMouser.rawValue
    case 7: return CGEventField.mouseEventSubtype.rawValue
    case 8: return CGEventField.keyboardEventAutorepeat.rawValue
    case 9: return CGEventField.keyboardEventKeycode.rawValue
    case 10: return CGEventField.keyboardEventKeyboardType.rawValue
    case 11: return CGEventField.scrollWheelEventDeltaAxis1.rawValue
    case 12: return CGEventField.scrollWheelEventDeltaAxis2.rawValue
    case 13: return CGEventField.scrollWheelEventDeltaAxis3.rawValue
    case 14: return CGEventField.scrollWheelEventFixedPtDeltaAxis1.rawValue
    case 15: return CGEventField.scrollWheelEventFixedPtDeltaAxis2.rawValue
    case 16: return CGEventField.scrollWheelEventFixedPtDeltaAxis3.rawValue
    case 17: return CGEventField.scrollWheelEventPointDeltaAxis1.rawValue
    case 18: return CGEventField.scrollWheelEventPointDeltaAxis2.rawValue
    case 19: return CGEventField.scrollWheelEventPointDeltaAxis3.rawValue
    case 20: return CGEventField.scrollWheelEventScrollPhase.rawValue
    case 21: return CGEventField.scrollWheelEventScrollCount.rawValue
    case 22: return CGEventField.scrollWheelEventMomentumPhase.rawValue
    case 23: return CGEventField.scrollWheelEventInstantMouser.rawValue
    case 24: return CGEventField.tabletEventPointX.rawValue
    case 25: return CGEventField.tabletEventPointY.rawValue
    case 26: return CGEventField.tabletEventPointZ.rawValue
    case 27: return CGEventField.tabletEventPointButtons.rawValue
    case 28: return CGEventField.tabletEventPointPressure.rawValue
    case 29: return CGEventField.tabletEventTiltX.rawValue
    case 30: return CGEventField.tabletEventTiltY.rawValue
    case 31: return CGEventField.tabletEventRotation.rawValue
    case 32: return CGEventField.tabletEventTangentialPressure.rawValue
    case 33: return CGEventField.tabletEventDeviceID.rawValue
    case 34: return CGEventField.tabletEventVendor1.rawValue
    case 35: return CGEventField.tabletEventVendor2.rawValue
    case 36: return CGEventField.tabletEventVendor3.rawValue
    case 37: return CGEventField.tabletProximityEventVendorID.rawValue
    case 38: return CGEventField.tabletProximityEventTabletID.rawValue
    case 39: return CGEventField.tabletProximityEventPointerID.rawValue
    case 40: return CGEventField.tabletProximityEventDeviceID.rawValue
    case 41: return CGEventField.tabletProximityEventSystemTabletID.rawValue
    case 42: return CGEventField.tabletProximityEventVendorPointerType.rawValue
    case 43: return CGEventField.tabletProximityEventVendorPointerSerialNumber.rawValue
    case 44: return CGEventField.tabletProximityEventVendorUniqueID.rawValue
    case 45: return CGEventField.tabletProximityEventCapabilityMask.rawValue
    case 46: return CGEventField.tabletProximityEventPointerType.rawValue
    case 47: return CGEventField.tabletProximityEventEnterProximity.rawValue
    case 48: return CGEventField.eventTargetProcessSerialNumber.rawValue
    case 49: return CGEventField.eventTargetUnixProcessID.rawValue
    case 50: return CGEventField.eventSourceUnixProcessID.rawValue
    case 51: return CGEventField.eventSourceUserData.rawValue
    case 52: return CGEventField.eventSourceUserID.rawValue
    case 53: return CGEventField.eventSourceGroupID.rawValue
    case 54: return CGEventField.eventSourceStateID.rawValue
    case 55: return CGEventField.scrollWheelEventIsContinuous.rawValue
    case 56: return CGEventField.mouseEventWindowUnderMousePointer.rawValue
    case 57: return CGEventField.mouseEventWindowUnderMousePointerThatCanHandleThisEvent.rawValue
    case 58: return CGEventField.eventUnacceleratedPointerMovementX.rawValue
    case 59: return CGEventField.eventUnacceleratedPointerMovementY.rawValue
    case 60: return CGEventField.scrollWheelEventMomentumOptionPhase.rawValue
    case 61: return CGEventField.scrollWheelEventAcceleratedDeltaAxis1.rawValue
    case 62: return CGEventField.scrollWheelEventAcceleratedDeltaAxis2.rawValue
    case 63: return CGEventField.scrollWheelEventRawDeltaAxis1.rawValue
    case 64: return CGEventField.scrollWheelEventRawDeltaAxis2.rawValue
    default: return UInt32.max
    }
}
