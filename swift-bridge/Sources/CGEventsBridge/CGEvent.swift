import CoreGraphics
import Foundation

@_cdecl("cgevent_get_type_id")
public func cgeventGetTypeID() -> UInt {
    CGEvent.typeID
}

@_cdecl("cgevent_create")
public func cgeventCreate(source: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let event = CGEvent(source: sourceFromHandle(source)) else { return nil }
    return retain(EventHolder(event))
}

@_cdecl("cgevent_create_data_length")
public func cgeventCreateDataLength(event: UnsafeMutableRawPointer?) -> Int {
    guard let event = eventFromHandle(event) else { return 0 }
    if #available(macOS 12.0, *) {
        guard let data = event.data else { return 0 }
        return (data as Data).count
    }
    return 0
}

@_cdecl("cgevent_create_data_copy")
public func cgeventCreateDataCopy(
    event: UnsafeMutableRawPointer?,
    buffer: UnsafeMutablePointer<UInt8>?,
    bufferSize: Int
) -> Bool {
    guard let event = eventFromHandle(event) else { return false }
    if #available(macOS 12.0, *) {
        guard let data = event.data else { return false }
        return copyBytes(data as Data, to: buffer, bufferSize: bufferSize)
    }
    return false
}

@_cdecl("cgevent_create_from_data")
public func cgeventCreateFromData(bytes: UnsafePointer<UInt8>?, length: Int) -> UnsafeMutableRawPointer? {
    guard length >= 0 else { return nil }
    let data = length == 0 ? Data() : Data(bytes: bytes!, count: length)
    guard let event = CGEvent(withDataAllocator: nil, data: data as CFData) else { return nil }
    return retain(EventHolder(event))
}

@_cdecl("cgevent_create_mouse_event")
public func cgeventCreateMouseEvent(
    source: UnsafeMutableRawPointer?,
    mouseType: UInt32,
    x: Double,
    y: Double,
    mouseButton: UInt32
) -> UnsafeMutableRawPointer? {
    guard let mouseType = CGEventType(rawValue: mouseType), let mouseButton = CGMouseButton(rawValue: mouseButton) else {
        return nil
    }
    guard let event = CGEvent(
        mouseEventSource: sourceFromHandle(source),
        mouseType: mouseType,
        mouseCursorPosition: CGPoint(x: x, y: y),
        mouseButton: mouseButton
    ) else {
        return nil
    }
    return retain(EventHolder(event))
}

@_cdecl("cgevent_create_keyboard_event")
public func cgeventCreateKeyboardEvent(
    source: UnsafeMutableRawPointer?,
    keycode: UInt16,
    keyDown: Bool
) -> UnsafeMutableRawPointer? {
    guard let event = CGEvent(
        keyboardEventSource: sourceFromHandle(source),
        virtualKey: keycode,
        keyDown: keyDown
    ) else {
        return nil
    }
    return retain(EventHolder(event))
}

@_cdecl("cgevent_create_scroll_wheel_event")
public func cgeventCreateScrollWheelEvent(
    source: UnsafeMutableRawPointer?,
    units: UInt32,
    wheelCount: UInt32,
    wheel1: Int32,
    wheel2: Int32,
    wheel3: Int32
) -> UnsafeMutableRawPointer? {
    guard let units = CGScrollEventUnit(rawValue: units) else { return nil }
    guard let event = CGEvent(
        scrollWheelEvent2Source: sourceFromHandle(source),
        units: units,
        wheelCount: wheelCount,
        wheel1: wheel1,
        wheel2: wheel2,
        wheel3: wheel3
    ) else {
        return nil
    }
    return retain(EventHolder(event))
}

@_cdecl("cgevent_create_copy")
public func cgeventCreateCopy(event: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let event = eventFromHandle(event) else { return nil }
    guard let copy = event.copy() else { return nil }
    return retain(EventHolder(copy))
}

@_cdecl("cgevent_create_source_from_event")
public func cgeventCreateSourceFromEvent(event: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let event = eventFromHandle(event) else { return nil }
    guard let source = CGEventSource(event: event) else { return nil }
    return retain(EventSourceHolder(source))
}

@_cdecl("cgevent_set_source")
public func cgeventSetSource(event: UnsafeMutableRawPointer?, source: UnsafeMutableRawPointer?) {
    guard let event = eventFromHandle(event) else { return }
    event.setSource(sourceFromHandle(source))
}

@_cdecl("cgevent_get_type")
public func cgeventGetType(event: UnsafeMutableRawPointer?) -> UInt32 {
    guard let event = eventFromHandle(event) else { return CGEventType.null.rawValue }
    return event.type.rawValue
}

@_cdecl("cgevent_set_type")
public func cgeventSetType(event: UnsafeMutableRawPointer?, type: UInt32) {
    guard let event = eventFromHandle(event), let type = CGEventType(rawValue: type) else { return }
    event.type = type
}

@_cdecl("cgevent_get_timestamp")
public func cgeventGetTimestamp(event: UnsafeMutableRawPointer?) -> UInt64 {
    guard let event = eventFromHandle(event) else { return 0 }
    return event.timestamp
}

@_cdecl("cgevent_set_timestamp")
public func cgeventSetTimestamp(event: UnsafeMutableRawPointer?, timestamp: UInt64) {
    eventFromHandle(event)?.timestamp = timestamp
}

@_cdecl("cgevent_get_location")
public func cgeventGetLocation(
    event: UnsafeMutableRawPointer?,
    outX: UnsafeMutablePointer<Double>?,
    outY: UnsafeMutablePointer<Double>?
) {
    guard let event = eventFromHandle(event), let outX, let outY else { return }
    outX.pointee = event.location.x
    outY.pointee = event.location.y
}

@_cdecl("cgevent_get_unflipped_location")
public func cgeventGetUnflippedLocation(
    event: UnsafeMutableRawPointer?,
    outX: UnsafeMutablePointer<Double>?,
    outY: UnsafeMutablePointer<Double>?
) {
    guard let event = eventFromHandle(event), let outX, let outY else { return }
    outX.pointee = event.unflippedLocation.x
    outY.pointee = event.unflippedLocation.y
}

@_cdecl("cgevent_set_location")
public func cgeventSetLocation(event: UnsafeMutableRawPointer?, x: Double, y: Double) {
    eventFromHandle(event)?.location = CGPoint(x: x, y: y)
}

@_cdecl("cgevent_get_flags")
public func cgeventGetFlags(event: UnsafeMutableRawPointer?) -> UInt64 {
    guard let event = eventFromHandle(event) else { return 0 }
    return event.flags.rawValue
}

@_cdecl("cgevent_set_flags")
public func cgeventSetFlags(event: UnsafeMutableRawPointer?, flags: UInt64) {
    eventFromHandle(event)?.flags = CGEventFlags(rawValue: flags)
}

@_cdecl("cgevent_get_integer_value_field")
public func cgeventGetIntegerValueField(event: UnsafeMutableRawPointer?, field: UInt32) -> Int64 {
    guard let event = eventFromHandle(event), let field = CGEventField(rawValue: field) else { return 0 }
    return event.getIntegerValueField(field)
}

@_cdecl("cgevent_set_integer_value_field")
public func cgeventSetIntegerValueField(event: UnsafeMutableRawPointer?, field: UInt32, value: Int64) {
    guard let event = eventFromHandle(event), let field = CGEventField(rawValue: field) else { return }
    event.setIntegerValueField(field, value: value)
}

@_cdecl("cgevent_get_double_value_field")
public func cgeventGetDoubleValueField(event: UnsafeMutableRawPointer?, field: UInt32) -> Double {
    guard let event = eventFromHandle(event), let field = CGEventField(rawValue: field) else { return 0 }
    return event.getDoubleValueField(field)
}

@_cdecl("cgevent_set_double_value_field")
public func cgeventSetDoubleValueField(event: UnsafeMutableRawPointer?, field: UInt32, value: Double) {
    guard let event = eventFromHandle(event), let field = CGEventField(rawValue: field) else { return }
    event.setDoubleValueField(field, value: value)
}

@_cdecl("cgevent_keyboard_set_unicode_string")
public func cgeventKeyboardSetUnicodeString(
    event: UnsafeMutableRawPointer?,
    utf16: UnsafePointer<UInt16>?,
    length: Int
) {
    guard let event = eventFromHandle(event), length >= 0 else { return }
    let buffer: [UniChar]
    if length == 0 {
        buffer = []
    } else if let utf16 {
        buffer = Array(UnsafeBufferPointer(start: utf16, count: length))
    } else {
        return
    }
    event.keyboardSetUnicodeString(stringLength: length, unicodeString: buffer)
}

@_cdecl("cgevent_keyboard_get_unicode_string_length")
public func cgeventKeyboardGetUnicodeStringLength(event: UnsafeMutableRawPointer?) -> Int {
    guard let event = eventFromHandle(event) else { return 0 }
    var actual = 0
    event.keyboardGetUnicodeString(maxStringLength: 0, actualStringLength: &actual, unicodeString: nil)
    return actual
}

@_cdecl("cgevent_keyboard_get_unicode_string")
public func cgeventKeyboardGetUnicodeString(
    event: UnsafeMutableRawPointer?,
    buffer: UnsafeMutablePointer<UInt16>?,
    bufferLength: Int
) -> Bool {
    guard let event = eventFromHandle(event), let buffer, bufferLength >= 0 else { return false }
    var actual = 0
    event.keyboardGetUnicodeString(maxStringLength: bufferLength, actualStringLength: &actual, unicodeString: buffer)
    return actual <= bufferLength
}

@_cdecl("cgevent_post")
public func cgeventPost(event: UnsafeMutableRawPointer?, tapLocation: UInt32) {
    guard let event = eventFromHandle(event), let tap = CGEventTapLocation(rawValue: tapLocation) else { return }
    event.post(tap: tap)
}

@_cdecl("cgevent_post_to_pid")
public func cgeventPostToPid(event: UnsafeMutableRawPointer?, pid: Int32) {
    guard let event = eventFromHandle(event) else { return }
    event.postToPid(pid)
}

@_cdecl("cgevent_release")
public func cgeventRelease(event: UnsafeMutableRawPointer?) {
    release(event)
}
