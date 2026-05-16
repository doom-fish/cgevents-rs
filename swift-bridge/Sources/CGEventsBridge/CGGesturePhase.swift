import CoreGraphics

@_cdecl("cggesture_phase_raw_value")
public func cggesturePhaseRawValue(index: UInt32) -> UInt32 {
    switch index {
    case 0: return CGGesturePhase.none.rawValue
    case 1: return CGGesturePhase.began.rawValue
    case 2: return CGGesturePhase.changed.rawValue
    case 3: return CGGesturePhase.ended.rawValue
    case 4: return CGGesturePhase.cancelled.rawValue
    case 5: return CGGesturePhase.mayBegin.rawValue
    default: return UInt32.max
    }
}
