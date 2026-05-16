import CoreGraphics

@_cdecl("cgmomentum_scroll_phase_raw_value")
public func cgmomentumScrollPhaseRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGMomentumScrollPhase(rawValue: 0)!.rawValue
    case 1: return CGMomentumScrollPhase.begin.rawValue
    case 2: return CGMomentumScrollPhase(rawValue: 2)!.rawValue
    case 3: return CGMomentumScrollPhase.end.rawValue
    default: return UInt32.max
    }
}
