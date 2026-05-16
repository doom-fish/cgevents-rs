import CoreGraphics

@_cdecl("cgscroll_phase_raw_value")
public func cgscrollPhaseRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGScrollPhase.began.rawValue
    case 1: return CGScrollPhase.changed.rawValue
    case 2: return CGScrollPhase.ended.rawValue
    case 3: return CGScrollPhase.cancelled.rawValue
    case 4: return CGScrollPhase.mayBegin.rawValue
    default: return UInt32.max
    }
}
