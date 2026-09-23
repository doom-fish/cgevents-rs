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
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32

public typealias ContextRetainCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

public typealias ContextReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

let tapActionPass: Int32 = 0
let tapActionDrop: Int32 = 1
let tapActionReplace: Int32 = 2
let tapActionReenable: Int32 = 3

struct TapState {
    let callback: RustTapCallback
    let context: UnsafeMutableRawPointer?
    let contextRelease: ContextReleaseCallback
    var port: Unmanaged<CFMachPort>?
    var dispatchDepth = 0
    var finalizePending = false
}

func finalizeTapState(_ state: UnsafeMutablePointer<TapState>) {
    let context = state.pointee.context
    let contextRelease = state.pointee.contextRelease
    state.deinitialize(count: 1)
    state.deallocate()
    contextRelease(context)
}

func finalizeTapStateWhenIdle(_ state: UnsafeMutablePointer<TapState>) {
    if state.pointee.dispatchDepth > 0 {
        state.pointee.finalizePending = true
    } else {
        finalizeTapState(state)
    }
}

final class EventTapHolder {
    let runLoop: CFRunLoop
    let port: CFMachPort
    let runLoopSource: CFRunLoopSource
    private let state: UnsafeMutablePointer<TapState>

    private init(
        runLoop: CFRunLoop,
        port: CFMachPort,
        runLoopSource: CFRunLoopSource,
        state: UnsafeMutablePointer<TapState>
    ) {
        self.runLoop = runLoop
        self.port = port
        self.runLoopSource = runLoopSource
        self.state = state
    }

    static func install(
        callback: @escaping RustTapCallback,
        context: UnsafeMutableRawPointer?,
        contextRetain: ContextRetainCallback,
        contextRelease: @escaping ContextReleaseCallback,
        createPort: (UnsafeMutableRawPointer) -> CFMachPort?
    ) -> EventTapHolder? {
        guard let runLoop = CFRunLoopGetCurrent() else {
            return nil
        }
        let state = UnsafeMutablePointer<TapState>.allocate(capacity: 1)
        state.initialize(to: TapState(callback: callback, context: context, contextRelease: contextRelease))
        contextRetain(context)
        guard let port = createPort(UnsafeMutableRawPointer(state)) else {
            finalizeTapState(state)
            return nil
        }
        guard let runLoopSource = CFMachPortCreateRunLoopSource(nil, port, 0) else {
            CFMachPortInvalidate(port)
            finalizeTapState(state)
            return nil
        }
        state.pointee.port = Unmanaged.passUnretained(port)
        CFRunLoopAddSource(runLoop, runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: port, enable: true)
        return EventTapHolder(runLoop: runLoop, port: port, runLoopSource: runLoopSource, state: state)
    }

    func teardown() {
        CGEvent.tapEnable(tap: port, enable: false)
        CFRunLoopRemoveSource(runLoop, runLoopSource, .commonModes)
        CFMachPortInvalidate(port)
        let state = self.state
        if runLoop === CFRunLoopGetCurrent() || CFRunLoopCopyCurrentMode(runLoop) == nil {
            finalizeTapStateWhenIdle(state)
            return
        }
        let port = self.port
        let finalized = DispatchSemaphore(value: 0)
        CFRunLoopPerformBlock(runLoop, CFRunLoopMode.commonModes.rawValue) {
            withExtendedLifetime(port) {
                finalizeTapStateWhenIdle(state)
            }
            finalized.signal()
        }
        CFRunLoopWakeUp(runLoop)
        _ = finalized.wait(timeout: .now() + .seconds(2))
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
