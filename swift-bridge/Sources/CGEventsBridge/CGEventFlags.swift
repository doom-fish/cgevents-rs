import CoreGraphics

@_cdecl("cgevent_flags_raw_value")
public func cgeventFlagsRawValue(index: UInt32) -> UInt64 {
    switch index {
    case 0: return CGEventFlags.maskAlphaShift.rawValue
    case 1: return CGEventFlags.maskShift.rawValue
    case 2: return CGEventFlags.maskControl.rawValue
    case 3: return CGEventFlags.maskAlternate.rawValue
    case 4: return CGEventFlags.maskCommand.rawValue
    case 5: return CGEventFlags.maskHelp.rawValue
    case 6: return CGEventFlags.maskSecondaryFn.rawValue
    case 7: return CGEventFlags.maskNumericPad.rawValue
    case 8: return CGEventFlags.maskNonCoalesced.rawValue
    default: return 0
    }
}

@_cdecl("cgevent_flags_known_mask")
public func cgeventFlagsKnownMask() -> UInt64 {
    CGEventFlags.maskAlphaShift.rawValue |
        CGEventFlags.maskShift.rawValue |
        CGEventFlags.maskControl.rawValue |
        CGEventFlags.maskAlternate.rawValue |
        CGEventFlags.maskCommand.rawValue |
        CGEventFlags.maskHelp.rawValue |
        CGEventFlags.maskSecondaryFn.rawValue |
        CGEventFlags.maskNumericPad.rawValue |
        CGEventFlags.maskNonCoalesced.rawValue
}
