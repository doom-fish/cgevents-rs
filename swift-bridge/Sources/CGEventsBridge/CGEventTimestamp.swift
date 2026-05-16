import CoreGraphics

@_cdecl("cgevent_timestamp_type_size")
public func cgeventTimestampTypeSize() -> Int {
    MemoryLayout<CGEventTimestamp>.size
}
