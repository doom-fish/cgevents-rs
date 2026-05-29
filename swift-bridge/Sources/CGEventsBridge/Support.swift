import CoreFoundation
import CoreGraphics
import Foundation

@frozen
public struct FFIPoint {
    public var x: Double
    public var y: Double

    public init(x: Double, y: Double) {
        self.x = x
        self.y = y
    }

    public init(_ point: CGPoint) {
        x = point.x
        y = point.y
    }
}

@frozen
public struct FFIEventTapInformation {
    public var eventTapID: UInt32
    public var tapPoint: UInt32
    public var options: UInt32
    public var eventsOfInterest: UInt64
    public var tappingProcess: Int32
    public var processBeingTapped: Int32
    public var enabled: Bool
    public var minUsecLatency: Float
    public var avgUsecLatency: Float
    public var maxUsecLatency: Float

    public init(_ info: CGEventTapInformation) {
        eventTapID = info.eventTapID
        tapPoint = info.tapPoint.rawValue
        options = info.options.rawValue
        eventsOfInterest = info.eventsOfInterest
        tappingProcess = info.tappingProcess
        processBeingTapped = info.processBeingTapped
        enabled = info.enabled
        minUsecLatency = info.minUsecLatency
        avgUsecLatency = info.avgUsecLatency
        maxUsecLatency = info.maxUsecLatency
    }
}

/// Cross-language ABI check called from Rust's `tests/ffi_layout_tests.rs`.
///
/// Returns `true` only if the Swift `MemoryLayout` (size, stride and alignment)
/// of `FFIEventTapInformation` matches the values pinned on the Rust side via
/// the `const _: () = assert!(...)` checks in `src/ffi/mod.rs`. If the layouts
/// ever drift apart this returns `false` and the Rust test fails, flagging a
/// real ABI mismatch.
@_cdecl("cgevent_verify_ffi_layout")
public func cgeventVerifyFFILayout() -> Bool {
    MemoryLayout<FFIEventTapInformation>.size == 48
        && MemoryLayout<FFIEventTapInformation>.stride == 48
        && MemoryLayout<FFIEventTapInformation>.alignment == 8
}

final class EventHolder {
    let event: CGEvent

    init(_ event: CGEvent) {
        self.event = event
    }
}

final class BorrowedEventHolder {
    let event: CGEvent

    init(_ event: CGEvent) {
        self.event = event
    }
}

final class EventSourceHolder {
    let source: CGEventSource

    init(_ source: CGEventSource) {
        self.source = source
    }
}

public typealias RustTapCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UInt32,
    UnsafeMutableRawPointer?
) -> Int32

/// C trampoline that takes a +1 reference on the Rust tap context.
public typealias ContextRetainCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

/// C trampoline that drops a reference on the Rust tap context.
public typealias ContextReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class EventTapHolder {
    let callback: RustTapCallback
    let context: UnsafeMutableRawPointer?
    let contextRelease: ContextReleaseCallback
    let runLoop: CFRunLoop
    var port: CFMachPort?
    var runLoopSource: CFRunLoopSource?

    init(
        callback: @escaping RustTapCallback,
        context: UnsafeMutableRawPointer?,
        contextRetain: ContextRetainCallback,
        contextRelease: @escaping ContextReleaseCallback,
        runLoop: CFRunLoop
    ) {
        self.callback = callback
        self.context = context
        self.contextRelease = contextRelease
        self.runLoop = runLoop
        // Take a +1 on the Rust TapInner for the lifetime of this holder. ARC
        // keeps this holder alive for the duration of each tap callback, so the
        // Rust context can never be freed while a callback is in flight — even
        // if the owning Rust `EventTap` is dropped from another thread.
        contextRetain(context)
    }

    static func create(
        location: CGEventTapLocation,
        placement: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: @escaping RustTapCallback,
        context: UnsafeMutableRawPointer?,
        contextRetain: ContextRetainCallback,
        contextRelease: @escaping ContextReleaseCallback
    ) -> EventTapHolder? {
        let holder = EventTapHolder(
            callback: callback,
            context: context,
            contextRetain: contextRetain,
            contextRelease: contextRelease,
            runLoop: CFRunLoopGetCurrent()
        )
        let userInfo = Unmanaged.passUnretained(holder).toOpaque()
        guard let port = CGEvent.tapCreate(
            tap: location,
            place: placement,
            options: options,
            eventsOfInterest: eventsOfInterest,
            callback: swiftTapCallback,
            userInfo: userInfo
        ) else {
            return nil
        }
        guard let runLoopSource = CFMachPortCreateRunLoopSource(nil, port, 0) else {
            CFMachPortInvalidate(port)
            return nil
        }
        holder.port = port
        holder.runLoopSource = runLoopSource
        CFRunLoopAddSource(holder.runLoop, runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: port, enable: true)
        return holder
    }

    static func createForPid(
        pid: Int32,
        placement: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: @escaping RustTapCallback,
        context: UnsafeMutableRawPointer?,
        contextRetain: ContextRetainCallback,
        contextRelease: @escaping ContextReleaseCallback
    ) -> EventTapHolder? {
        let holder = EventTapHolder(
            callback: callback,
            context: context,
            contextRetain: contextRetain,
            contextRelease: contextRelease,
            runLoop: CFRunLoopGetCurrent()
        )
        let userInfo = Unmanaged.passUnretained(holder).toOpaque()
        guard let port = CGEvent.tapCreateForPid(
            pid: pid,
            place: placement,
            options: options,
            eventsOfInterest: eventsOfInterest,
            callback: swiftTapCallback,
            userInfo: userInfo
        ) else {
            return nil
        }
        guard let runLoopSource = CFMachPortCreateRunLoopSource(nil, port, 0) else {
            CFMachPortInvalidate(port)
            return nil
        }
        holder.port = port
        holder.runLoopSource = runLoopSource
        CFRunLoopAddSource(holder.runLoop, runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: port, enable: true)
        return holder
    }

    deinit {
        // ARC defers this `deinit` until no callback is in flight (each callback
        // holds a strong reference to the holder for its duration), so removing
        // the run-loop source and invalidating the port here cannot race with a
        // running callback. We drop the Rust context reference last, balancing
        // the `contextRetain` taken in `init`.
        if let runLoopSource {
            CFRunLoopRemoveSource(runLoop, runLoopSource, .commonModes)
        }
        if let port {
            CFMachPortInvalidate(port)
        }
        contextRelease(context)
    }
}

func retain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

func unretainedAny(_ ptr: UnsafeMutableRawPointer?) -> AnyObject? {
    guard let ptr else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
}

func eventFromHandle(_ ptr: UnsafeMutableRawPointer?) -> CGEvent? {
    guard let object = unretainedAny(ptr) else { return nil }
    if let holder = object as? EventHolder {
        return holder.event
    }
    if let holder = object as? BorrowedEventHolder {
        return holder.event
    }
    return nil
}

func sourceFromHandle(_ ptr: UnsafeMutableRawPointer?) -> CGEventSource? {
    (unretainedAny(ptr) as? EventSourceHolder)?.source
}

func tapHolderFromHandle(_ ptr: UnsafeMutableRawPointer?) -> EventTapHolder? {
    unretainedAny(ptr) as? EventTapHolder
}

func makeBorrowedEventHandle(_ event: CGEvent) -> UnsafeMutableRawPointer {
    retain(BorrowedEventHolder(event))
}

func copyBytes(_ data: Data, to buffer: UnsafeMutablePointer<UInt8>?, bufferSize: Int) -> Bool {
    guard let buffer else { return false }
    guard data.count <= bufferSize else { return false }
    data.copyBytes(to: buffer, count: data.count)
    return true
}

func pointFromFFI(_ point: FFIPoint) -> CGPoint {
    CGPoint(x: point.x, y: point.y)
}
