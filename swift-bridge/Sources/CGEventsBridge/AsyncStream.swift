import CoreFoundation
import CoreGraphics
import Foundation

/// C-callable callback type delivered by the async tap bridge.
/// Parameters: event_type, location_x, location_y, flags, timestamp, keycode, ctx.
public typealias CGEventsStreamCallback = @convention(c) (
    UInt32,
    Double,
    Double,
    UInt64,
    UInt64,
    UInt16,
    UnsafeMutableRawPointer?
) -> Void

/// Bridge that owns a dedicated run-loop thread for a listen-only CGEvent tap.
///
/// The tap is created on the dedicated thread's run loop so that the thread
/// blocks in `CFRunLoopRun()` without interfering with the caller's run loop.
/// On `deinit`, the tap is disabled/invalidated and the run loop is stopped,
/// which causes the thread to exit.
final class CGEventsTapAsyncStreamBridge {
    let callback: CGEventsStreamCallback
    let ctx: UnsafeMutableRawPointer?
    var port: CFMachPort?
    var runLoopSource: CFRunLoopSource?
    var runLoop: CFRunLoop?

    init(callback: @escaping CGEventsStreamCallback, ctx: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.ctx = ctx
    }

    /// Create the bridge: spawn the run-loop thread, install the tap, wait for readiness.
    ///
    /// Returns `nil` if the tap could not be created (e.g., missing Accessibility
    /// permission).
    static func create(
        location: CGEventTapLocation,
        eventsOfInterest: CGEventMask,
        callback: @escaping CGEventsStreamCallback,
        ctx: UnsafeMutableRawPointer?
    ) -> CGEventsTapAsyncStreamBridge? {
        let bridge = CGEventsTapAsyncStreamBridge(callback: callback, ctx: ctx)
        let sema = DispatchSemaphore(value: 0)

        // Use [weak bridge] so the thread closure does not keep the bridge alive
        // after the `Unmanaged.passRetained` handle has been released.
        let thread = Thread { [weak bridge] in
            guard let bridge = bridge else {
                sema.signal()
                return
            }
            let rl = CFRunLoopGetCurrent()!
            bridge.runLoop = rl

            let userInfo = Unmanaged.passUnretained(bridge).toOpaque()
            guard let port = CGEvent.tapCreate(
                tap: location,
                place: .headInsertEventTap,
                options: .listenOnly,
                eventsOfInterest: eventsOfInterest,
                callback: cgeventsAsyncTapCallback,
                userInfo: userInfo
            ) else {
                sema.signal()
                return
            }
            guard let src = CFMachPortCreateRunLoopSource(nil, port, 0) else {
                CFMachPortInvalidate(port)
                sema.signal()
                return
            }
            bridge.port = port
            bridge.runLoopSource = src
            CFRunLoopAddSource(rl, src, .commonModes)
            CGEvent.tapEnable(tap: port, enable: true)
            sema.signal() // signal *after* tap is fully installed

            CFRunLoopRun() // blocks until deinit calls CFRunLoopStop
        }
        thread.name = "com.doom-fish.cgevents.async-tap"
        thread.start()
        sema.wait()

        guard bridge.port != nil else { return nil }
        return bridge
    }

    deinit {
        if let port = port {
            CGEvent.tapEnable(tap: port, enable: false)
            CFMachPortInvalidate(port)
        }
        if let src = runLoopSource, let rl = runLoop {
            CFRunLoopRemoveSource(rl, src, .commonModes)
        }
        if let rl = runLoop {
            // Wake and stop the run-loop thread.
            CFRunLoopStop(rl)
        }
    }
}

/// CGEvent tap callback wired into `CGEventsTapAsyncStreamBridge`.
///
/// Captures the event fields into scalar values and forwards them to the
/// Rust `CGEventsStreamCallback` stored in the bridge.
private func cgeventsAsyncTapCallback(
    proxy: CGEventTapProxy,
    type: CGEventType,
    event: CGEvent,
    userInfo: UnsafeMutableRawPointer?
) -> Unmanaged<CGEvent>? {
    guard let userInfo = userInfo else {
        return Unmanaged.passUnretained(event)
    }
    let bridge = Unmanaged<CGEventsTapAsyncStreamBridge>
        .fromOpaque(userInfo)
        .takeUnretainedValue()
    let location = event.location
    let flags = event.flags.rawValue
    let timestamp = event.timestamp
    let keycode = UInt16(
        truncatingIfNeeded: event.getIntegerValueField(.keyboardEventKeycode))
    bridge.callback(
        type.rawValue,
        location.x,
        location.y,
        flags,
        timestamp,
        keycode,
        bridge.ctx
    )
    // Listen-only tap: always pass the event through unchanged.
    return Unmanaged.passUnretained(event)
}

/// Install a listen-only CGEvent tap on a dedicated run-loop thread and
/// return an opaque handle.
///
/// Every event matching `eventsOfInterest` will invoke `callback(event_type,
/// location_x, location_y, flags, timestamp, keycode, ctx)` from the tap
/// thread. The handle must be released with `cgevents_tap_stream_unsubscribe`
/// when the stream is no longer needed.
///
/// Returns `nil` if the tap could not be created.
@_cdecl("cgevents_tap_stream_subscribe")
public func cgeventsTapStreamSubscribe(
    location: UInt32,
    eventsOfInterest: UInt64,
    callback: CGEventsStreamCallback,
    ctx: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let location = CGEventTapLocation(rawValue: location) else { return nil }
    guard let bridge = CGEventsTapAsyncStreamBridge.create(
        location: location,
        eventsOfInterest: eventsOfInterest,
        callback: callback,
        ctx: ctx
    ) else {
        return nil
    }
    return Unmanaged.passRetained(bridge).toOpaque()
}

/// Release the async tap bridge returned by `cgevents_tap_stream_subscribe`.
///
/// This disables the tap, invalidates the mach port, and stops the
/// dedicated run-loop thread. After this call, no more callbacks will be
/// delivered.
@_cdecl("cgevents_tap_stream_unsubscribe")
public func cgeventsTapStreamUnsubscribe(handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<CGEventsTapAsyncStreamBridge>.fromOpaque(handle).release()
}
