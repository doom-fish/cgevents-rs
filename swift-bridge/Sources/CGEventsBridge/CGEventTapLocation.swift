import CoreGraphics

@_cdecl("cgevent_tap_location_raw_value")
public func cgeventTapLocationRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGEventTapLocation.cghidEventTap.rawValue
    case 1: return CGEventTapLocation.cgSessionEventTap.rawValue
    case 2: return CGEventTapLocation.cgAnnotatedSessionEventTap.rawValue
    default: return UInt32.max
    }
}
