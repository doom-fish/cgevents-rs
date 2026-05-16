import CoreGraphics

@_cdecl("cgevent_tap_proxy_post_event")
public func cgeventTapProxyPostEvent(proxy: UnsafeMutableRawPointer?, event: UnsafeMutableRawPointer?) {
    guard let proxy, let event = eventFromHandle(event) else { return }
    event.tapPostEvent(OpaquePointer(proxy))
}
