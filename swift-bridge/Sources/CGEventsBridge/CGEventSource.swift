import CoreGraphics

@_cdecl("cgevent_source_get_type_id")
public func cgeventSourceGetTypeID() -> UInt {
    CGEventSource.typeID
}

@_cdecl("cgevent_source_create")
public func cgeventSourceCreate(stateID: Int32) -> UnsafeMutableRawPointer? {
    guard let stateID = CGEventSourceStateID(rawValue: stateID) else { return nil }
    guard let source = CGEventSource(stateID: stateID) else { return nil }
    return retain(EventSourceHolder(source))
}

@_cdecl("cgevent_source_get_keyboard_type")
public func cgeventSourceGetKeyboardType(source: UnsafeMutableRawPointer?) -> UInt32 {
    guard let source = sourceFromHandle(source) else { return 0 }
    return source.keyboardType
}

@_cdecl("cgevent_source_set_keyboard_type")
public func cgeventSourceSetKeyboardType(source: UnsafeMutableRawPointer?, keyboardType: UInt32) {
    sourceFromHandle(source)?.keyboardType = keyboardType
}

@_cdecl("cgevent_source_get_pixels_per_line")
public func cgeventSourceGetPixelsPerLine(source: UnsafeMutableRawPointer?) -> Double {
    guard let source = sourceFromHandle(source) else { return 0 }
    return source.pixelsPerLine
}

@_cdecl("cgevent_source_set_pixels_per_line")
public func cgeventSourceSetPixelsPerLine(source: UnsafeMutableRawPointer?, pixelsPerLine: Double) {
    sourceFromHandle(source)?.pixelsPerLine = pixelsPerLine
}

@_cdecl("cgevent_source_get_source_state_id")
public func cgeventSourceGetSourceStateID(source: UnsafeMutableRawPointer?) -> Int32 {
    guard let source = sourceFromHandle(source) else { return CGEventSourceStateID.combinedSessionState.rawValue }
    return source.sourceStateID.rawValue
}

@_cdecl("cgevent_source_button_state")
public func cgeventSourceButtonState(stateID: Int32, button: UInt32) -> Bool {
    guard let stateID = CGEventSourceStateID(rawValue: stateID), let button = CGMouseButton(rawValue: button) else {
        return false
    }
    return CGEventSource.buttonState(stateID, button: button)
}

@_cdecl("cgevent_source_key_state")
public func cgeventSourceKeyState(stateID: Int32, keycode: UInt16) -> Bool {
    guard let stateID = CGEventSourceStateID(rawValue: stateID) else { return false }
    return CGEventSource.keyState(stateID, key: keycode)
}

@_cdecl("cgevent_source_flags_state")
public func cgeventSourceFlagsState(stateID: Int32) -> UInt64 {
    guard let stateID = CGEventSourceStateID(rawValue: stateID) else { return 0 }
    return CGEventSource.flagsState(stateID).rawValue
}

@_cdecl("cgevent_source_seconds_since_last_event_type")
public func cgeventSourceSecondsSinceLastEventType(stateID: Int32, eventType: UInt32) -> Double {
    guard let stateID = CGEventSourceStateID(rawValue: stateID), let eventType = CGEventType(rawValue: eventType) else {
        return 0
    }
    return CGEventSource.secondsSinceLastEventType(stateID, eventType: eventType)
}

@_cdecl("cgevent_source_counter_for_event_type")
public func cgeventSourceCounterForEventType(stateID: Int32, eventType: UInt32) -> UInt32 {
    guard let stateID = CGEventSourceStateID(rawValue: stateID), let eventType = CGEventType(rawValue: eventType) else {
        return 0
    }
    return CGEventSource.counterForEventType(stateID, eventType: eventType)
}

@_cdecl("cgevent_source_set_user_data")
public func cgeventSourceSetUserData(source: UnsafeMutableRawPointer?, userData: Int64) {
    sourceFromHandle(source)?.userData = userData
}

@_cdecl("cgevent_source_get_user_data")
public func cgeventSourceGetUserData(source: UnsafeMutableRawPointer?) -> Int64 {
    guard let source = sourceFromHandle(source) else { return 0 }
    return source.userData
}

@_cdecl("cgevent_source_set_local_events_filter_during_suppression_state")
public func cgeventSourceSetLocalEventsFilterDuringSuppressionState(
    source: UnsafeMutableRawPointer?,
    filter: UInt32,
    state: UInt32
) {
    guard let source = sourceFromHandle(source), let state = CGEventSuppressionState(rawValue: state) else {
        return
    }
    source.setLocalEventsFilterDuringSuppressionState(CGEventFilterMask(rawValue: filter), state: state)
}

@_cdecl("cgevent_source_get_local_events_filter_during_suppression_state")
public func cgeventSourceGetLocalEventsFilterDuringSuppressionState(
    source: UnsafeMutableRawPointer?,
    state: UInt32
) -> UInt32 {
    guard let source = sourceFromHandle(source), let state = CGEventSuppressionState(rawValue: state) else {
        return 0
    }
    return source.getLocalEventsFilterDuringSuppressionState(state).rawValue
}

@_cdecl("cgevent_source_set_local_events_suppression_interval")
public func cgeventSourceSetLocalEventsSuppressionInterval(source: UnsafeMutableRawPointer?, seconds: Double) {
    sourceFromHandle(source)?.localEventsSuppressionInterval = seconds
}

@_cdecl("cgevent_source_get_local_events_suppression_interval")
public func cgeventSourceGetLocalEventsSuppressionInterval(source: UnsafeMutableRawPointer?) -> Double {
    guard let source = sourceFromHandle(source) else { return 0 }
    return source.localEventsSuppressionInterval
}

@_cdecl("cgevent_source_release")
public func cgeventSourceRelease(source: UnsafeMutableRawPointer?) {
    release(source)
}
