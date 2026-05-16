use super::{CGEventBridgeHandle, CGEventTapProxyBridgeHandle};

unsafe extern "C" {
    pub fn cgevent_tap_proxy_post_event(
        proxy: CGEventTapProxyBridgeHandle,
        event: CGEventBridgeHandle,
    );
}
