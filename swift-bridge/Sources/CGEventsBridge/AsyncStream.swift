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
/// Call `stopAndJoin()` before releasing the bridge to guarantee the run-loop
/// thread has fully exited and no more callbacks will fire.
final class CGEventsTapAsyncStreamBridge {
    let callback: CGEventsStreamCallback
    let ctx: UnsafeMutableRawPointer?
    var port: CFMachPort?
    var runLoopSource: CFRunLoopSource?
    var runLoop: CFRunLoop?
    /// Signalled by the tap thread after `CFRunLoopRun()` returns.
    private let threadExitSema = DispatchSemaphore(value: 0)

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

            CFRunLoopRun() // blocks until stopAndJoin() calls CFRunLoopStop

            // Notify stopAndJoin() that the run-loop thread has fully exited.
            bridge.threadExitSema.signal()
        }
        thread.name = "com.doom-fish.cgevents.async-tap"
        thread.start()
        sema.wait()

        guard bridge.port != nil else { return nil }
        return bridge
    }

    /// Disable the tap, stop the run-loop thread, and block until the thread
    /// has fully exited.  Must be called before releasing the bridge so that
    /// no in-flight callback can observe a freed Rust sender pointer.
    func stopAndJoin() {
        if let port = port {
            CGEvent.tapEnable(tap: port, enable: false)
            CFMachPortInvalidate(port)
        }
        if let src = runLoopSource, let rl = runLoop {
            CFRunLoopRemoveSource(rl, src, .commonModes)
        }
        if let rl = runLoop {
            // Signals CFRunLoopRun() to return on the tap thread.
            CFRunLoopStop(rl)
        }
        // Wait for the tap thread to fully exit before returning to the Rust
        // caller, which will immediately free the sender pointer.
        threadExitSema.wait()
    }

    deinit {
        // By the time deinit runs the tap thread has already exited (it held
        // the last strong reference that delayed deallocation), so stopAndJoin()
        // has necessarily been called already.  The cleanup calls below are a
        // defence-in-depth no-op guard against unexpected reference patterns.
        if let port = port {
            CGEvent.tapEnable(tap: port, enable: false)
            CFMachPortInvalidate(port)
        }
        if let src = runLoopSource, let rl = runLoop {
            CFRunLoopRemoveSource(rl, src, .commonModes)
        }
        if let rl = runLoop {
            CFRunLoopStop(rl)
        }
        // Do NOT call threadExitSema.wait() here — the thread has already
        // signalled it (it held the last strong ref that gated this deinit),
        // so waiting again would deadlock.
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
/// Calls `stopAndJoin()` first to disable the tap and block until the
/// dedicated run-loop thread has fully exited, guaranteeing that no further
/// callbacks will fire after this function returns.  Only then is the bridge
/// object released.
@_cdecl("cgevents_tap_stream_unsubscribe")
public func cgeventsTapStreamUnsubscribe(handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    let unmanaged = Unmanaged<CGEventsTapAsyncStreamBridge>.fromOpaque(handle)
    // Stop the run-loop thread and wait for it to exit before releasing the
    // bridge.  This ensures the Rust caller can safely free the sender pointer
    // immediately after this function returns.
    unmanaged.takeUnretainedValue().stopAndJoin()
    unmanaged.release()
}
