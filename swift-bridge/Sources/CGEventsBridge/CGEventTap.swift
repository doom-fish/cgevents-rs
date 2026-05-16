import CoreFoundation
import CoreGraphics

private func tapActionPasses(_ action: Int32) -> Bool {
    action == 0
}

private func proxyPointer(_ proxy: CGEventTapProxy) -> UnsafeMutableRawPointer? {
    UnsafeMutableRawPointer(proxy)
}

func swiftTapCallback(
    proxy: CGEventTapProxy,
    type: CGEventType,
    event: CGEvent,
    userInfo: UnsafeMutableRawPointer?
) -> Unmanaged<CGEvent>? {
    guard let holder = tapHolderFromHandle(userInfo) else {
        return Unmanaged.passUnretained(event)
    }
    let borrowedEvent = makeBorrowedEventHandle(event)
    let action = holder.callback(holder.context, proxyPointer(proxy), type.rawValue, borrowedEvent)
    release(borrowedEvent)
    return tapActionPasses(action) ? Unmanaged.passUnretained(event) : nil
}

@_cdecl("cgevent_tap_create")
public func cgeventTapCreate(
    location: UInt32,
    place: UInt32,
    options: UInt32,
    eventsOfInterest: UInt64,
    callback: @escaping RustTapCallback,
    context: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let location = CGEventTapLocation(rawValue: location), let place = CGEventTapPlacement(rawValue: place) else {
        return nil
    }
    guard let holder = EventTapHolder.create(
        location: location,
        placement: place,
        options: CGEventTapOptions(rawValue: options) ?? .defaultTap,
        eventsOfInterest: eventsOfInterest,
        callback: callback,
        context: context
    ) else {
        return nil
    }
    return retain(holder)
}

@_cdecl("cgevent_tap_create_for_pid")
public func cgeventTapCreateForPid(
    pid: Int32,
    place: UInt32,
    options: UInt32,
    eventsOfInterest: UInt64,
    callback: @escaping RustTapCallback,
    context: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let place = CGEventTapPlacement(rawValue: place) else { return nil }
    guard let holder = EventTapHolder.createForPid(
        pid: pid,
        placement: place,
        options: CGEventTapOptions(rawValue: options) ?? .defaultTap,
        eventsOfInterest: eventsOfInterest,
        callback: callback,
        context: context
    ) else {
        return nil
    }
    return retain(holder)
}

@_cdecl("cgevent_tap_enable")
public func cgeventTapEnable(tap: UnsafeMutableRawPointer?, enable: Bool) {
    guard let holder = tapHolderFromHandle(tap), let port = holder.port else { return }
    CGEvent.tapEnable(tap: port, enable: enable)
}

@_cdecl("cgevent_tap_is_enabled")
public func cgeventTapIsEnabled(tap: UnsafeMutableRawPointer?) -> Bool {
    guard let holder = tapHolderFromHandle(tap), let port = holder.port else { return false }
    return CGEvent.tapIsEnabled(tap: port)
}

@_cdecl("cgevent_tap_run_current_run_loop")
public func cgeventTapRunCurrentRunLoop() {
    CFRunLoopRun()
}

@_cdecl("cgevent_tap_stop_current_run_loop")
public func cgeventTapStopCurrentRunLoop() {
    CFRunLoopStop(CFRunLoopGetCurrent())
}

@_cdecl("cgevent_tap_stop")
public func cgeventTapStop(tap: UnsafeMutableRawPointer?) {
    guard let holder = tapHolderFromHandle(tap) else { return }
    CFRunLoopStop(holder.runLoop)
}

@_cdecl("cgevent_tap_release")
public func cgeventTapRelease(tap: UnsafeMutableRawPointer?) {
    release(tap)
}

@_cdecl("cgevent_get_event_tap_list")
public func cgeventGetEventTapList(
    maxNumberOfTaps: UInt32,
    tapList: UnsafeMutableRawPointer?,
    eventTapCount: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    if tapList == nil || maxNumberOfTaps == 0 {
        return Int32(CGGetEventTapList(0, nil, eventTapCount).rawValue)
    }

    var count = eventTapCount?.pointee ?? 0
    var nativeList = Array(repeating: CGEventTapInformation(), count: Int(maxNumberOfTaps))
    let error = nativeList.withUnsafeMutableBufferPointer { buffer in
        CGGetEventTapList(maxNumberOfTaps, buffer.baseAddress, &count)
    }
    eventTapCount?.pointee = count
    if error == .success, let tapList {
        let buffer = tapList.assumingMemoryBound(to: FFIEventTapInformation.self)
        for index in 0 ..< min(Int(count), nativeList.count) {
            buffer[index] = FFIEventTapInformation(nativeList[index])
        }
    }
    return Int32(error.rawValue)
}

@_cdecl("cgevent_preflight_listen_event_access")
public func cgeventPreflightListenEventAccess() -> Bool {
    if #available(macOS 10.15, *) {
        return CGPreflightListenEventAccess()
    }
    return true
}

@_cdecl("cgevent_request_listen_event_access")
public func cgeventRequestListenEventAccess() -> Bool {
    if #available(macOS 10.15, *) {
        return CGRequestListenEventAccess()
    }
    return true
}

@_cdecl("cgevent_preflight_post_event_access")
public func cgeventPreflightPostEventAccess() -> Bool {
    if #available(macOS 10.15, *) {
        return CGPreflightPostEventAccess()
    }
    return true
}

@_cdecl("cgevent_request_post_event_access")
public func cgeventRequestPostEventAccess() -> Bool {
    if #available(macOS 10.15, *) {
        return CGRequestPostEventAccess()
    }
    return true
}
