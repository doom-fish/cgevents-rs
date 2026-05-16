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

final class EventTapHolder {
    let callback: RustTapCallback
    let context: UnsafeMutableRawPointer?
    let runLoop: CFRunLoop
    var port: CFMachPort?
    var runLoopSource: CFRunLoopSource?

    init(callback: @escaping RustTapCallback, context: UnsafeMutableRawPointer?, runLoop: CFRunLoop) {
        self.callback = callback
        self.context = context
        self.runLoop = runLoop
    }

    static func create(
        location: CGEventTapLocation,
        placement: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: @escaping RustTapCallback,
        context: UnsafeMutableRawPointer?
    ) -> EventTapHolder? {
        let holder = EventTapHolder(callback: callback, context: context, runLoop: CFRunLoopGetCurrent())
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
        context: UnsafeMutableRawPointer?
    ) -> EventTapHolder? {
        let holder = EventTapHolder(callback: callback, context: context, runLoop: CFRunLoopGetCurrent())
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
        if let runLoopSource {
            CFRunLoopRemoveSource(runLoop, runLoopSource, .commonModes)
        }
        if let port {
            CFMachPortInvalidate(port)
        }
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
