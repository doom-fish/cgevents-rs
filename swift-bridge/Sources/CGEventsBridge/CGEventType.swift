import CoreGraphics

@_cdecl("cgevent_type_raw_value")
public func cgeventTypeRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGEventType.null.rawValue
    case 1: return CGEventType.leftMouseDown.rawValue
    case 2: return CGEventType.leftMouseUp.rawValue
    case 3: return CGEventType.rightMouseDown.rawValue
    case 4: return CGEventType.rightMouseUp.rawValue
    case 5: return CGEventType.mouseMoved.rawValue
    case 6: return CGEventType.leftMouseDragged.rawValue
    case 7: return CGEventType.rightMouseDragged.rawValue
    case 8: return CGEventType.keyDown.rawValue
    case 9: return CGEventType.keyUp.rawValue
    case 10: return CGEventType.flagsChanged.rawValue
    case 11: return CGEventType.scrollWheel.rawValue
    case 12: return CGEventType.tabletPointer.rawValue
    case 13: return CGEventType.tabletProximity.rawValue
    case 14: return CGEventType.otherMouseDown.rawValue
    case 15: return CGEventType.otherMouseUp.rawValue
    case 16: return CGEventType.otherMouseDragged.rawValue
    case 17: return CGEventType.tapDisabledByTimeout.rawValue
    case 18: return CGEventType.tapDisabledByUserInput.rawValue
    default: return UInt32.max
    }
}

@_cdecl("cgevent_type_mask_bit")
public func cgeventTypeMaskBit(type: UInt32) -> UInt64 {
    guard type < 64 else { return 0 }
    return UInt64(1) << UInt64(type)
}

@_cdecl("cgevent_mask_for_all_events")
public func cgeventMaskForAllEvents() -> UInt64 {
    ~UInt64(0)
}
