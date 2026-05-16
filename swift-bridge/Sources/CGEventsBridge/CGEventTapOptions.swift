import CoreGraphics

@_cdecl("cgevent_tap_options_raw_value")
public func cgeventTapOptionsRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGEventTapOptions.defaultTap.rawValue
    case 1: return CGEventTapOptions.listenOnly.rawValue
    default: return UInt32.max
    }
}
