import CoreFoundation
import CoreGraphics

private func proxyPointer(_ proxy: CGEventTapProxy) -> UnsafeMutableRawPointer? {
    UnsafeMutableRawPointer(proxy)
}

func swiftTapCallback(
    proxy: CGEventTapProxy,
    type: CGEventType,
    event: CGEvent,
    userInfo: UnsafeMutableRawPointer?
) -> Unmanaged<CGEvent>? {
    guard let userInfo else {
        return Unmanaged.passUnretained(event)
    }
    let state = userInfo.assumingMemoryBound(to: TapState.self)
    let callback = state.pointee.callback
    let context = state.pointee.context
    let borrowedEvent = makeBorrowedEventHandle(event)
    var replacement: UnsafeMutableRawPointer?
    state.pointee.dispatchDepth += 1
    let action = callback(context, proxyPointer(proxy), type.rawValue, borrowedEvent, &replacement)
    state.pointee.dispatchDepth -= 1
    release(borrowedEvent)

    if state.pointee.finalizePending {
        if state.pointee.dispatchDepth == 0 {
            finalizeTapState(state)
        }
    } else if action == tapActionReenable, let port = state.pointee.port {
        CGEvent.tapEnable(tap: port.takeUnretainedValue(), enable: true)
    }

    switch action {
    case tapActionDrop:
        return nil
    case tapActionReplace:
        guard let replacement else {
            return Unmanaged.passUnretained(event)
        }
        defer { release(replacement) }
        guard let newEvent = eventFromHandle(replacement) else {
            return Unmanaged.passUnretained(event)
        }
        return Unmanaged.passRetained(newEvent)
    default:
        return Unmanaged.passUnretained(event)
    }
}

@_cdecl("cgevent_tap_create")
public func cgeventTapCreate(
    location: UInt32,
    place: UInt32,
    options: UInt32,
    eventsOfInterest: UInt64,
    callback: @escaping RustTapCallback,
    context: UnsafeMutableRawPointer?,
    contextRetain: @escaping ContextRetainCallback,
    contextRelease: @escaping ContextReleaseCallback
) -> UnsafeMutableRawPointer? {
    guard let location = CGEventTapLocation(rawValue: location), let place = CGEventTapPlacement(rawValue: place) else {
        return nil
    }
    let options = CGEventTapOptions(rawValue: options) ?? .defaultTap
    guard let holder = EventTapHolder.install(
        callback: callback,
        context: context,
        contextRetain: contextRetain,
        contextRelease: contextRelease,
        createPort: { userInfo in
            CGEvent.tapCreate(
                tap: location,
                place: place,
                options: options,
                eventsOfInterest: eventsOfInterest,
                callback: swiftTapCallback,
                userInfo: userInfo)
        }
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
    context: UnsafeMutableRawPointer?,
    contextRetain: @escaping ContextRetainCallback,
    contextRelease: @escaping ContextReleaseCallback
) -> UnsafeMutableRawPointer? {
    guard let place = CGEventTapPlacement(rawValue: place) else { return nil }
    let options = CGEventTapOptions(rawValue: options) ?? .defaultTap
    guard let holder = EventTapHolder.install(
        callback: callback,
        context: context,
        contextRetain: contextRetain,
        contextRelease: contextRelease,
        createPort: { userInfo in
            CGEvent.tapCreateForPid(
                pid: pid,
                place: place,
                options: options,
                eventsOfInterest: eventsOfInterest,
                callback: swiftTapCallback,
                userInfo: userInfo)
        }
    ) else {
        return nil
    }
    return retain(holder)
}

@_cdecl("cgevent_tap_enable")
public func cgeventTapEnable(tap: UnsafeMutableRawPointer?, enable: Bool) {
    guard let holder = tapHolderFromHandle(tap) else { return }
    CGEvent.tapEnable(tap: holder.port, enable: enable)
}

@_cdecl("cgevent_tap_is_enabled")
public func cgeventTapIsEnabled(tap: UnsafeMutableRawPointer?) -> Bool {
    guard let holder = tapHolderFromHandle(tap) else { return false }
    return CGEvent.tapIsEnabled(tap: holder.port)
}

@_cdecl("cgevent_tap_run")
public func cgeventTapRun(tap: UnsafeMutableRawPointer?) -> Bool {
    guard let holder = tapHolderFromHandle(tap), holder.runLoop === CFRunLoopGetCurrent() else {
        return false
    }
    CFRunLoopRun()
    return true
}

@_cdecl("cgevent_tap_stop_current_run_loop")
public func cgeventTapStopCurrentRunLoop() {
    CFRunLoopStop(CFRunLoopGetCurrent())
}

@_cdecl("cgevent_tap_stop")
public func cgeventTapStop(tap: UnsafeMutableRawPointer?) {
    guard let holder = tapHolderFromHandle(tap) else { return }
    requestRunLoopStop(holder.runLoop)
}

@_cdecl("cgevent_tap_release")
public func cgeventTapRelease(tap: UnsafeMutableRawPointer?) {
    guard let holder = tapHolderFromHandle(tap) else { return }
    holder.teardown()
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
