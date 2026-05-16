import CoreGraphics

@_cdecl("cgevent_mouse_subtype_raw_value")
public func cgeventMouseSubtypeRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return 0
    case 1: return CGEventMouseSubtype.tabletPoint.rawValue
    case 2: return CGEventMouseSubtype.tabletProximity.rawValue
    default: return UInt32.max
    }
}
