//! `CGEventTap` — intercept (and optionally modify or drop) events as they flow through the system.

use core::ffi::c_void;
use core::ptr;
use std::sync::Mutex;

use crate::cg_event_field::CGEventField;
use crate::cg_event_flags::CGEventFlags;
use crate::cg_event_mouse_subtype::CGEventMouseSubtype;
use crate::cg_event_tap_location::{CGEventTapLocation, TapLocation};
use crate::cg_event_tap_options::CGEventTapOptions;
use crate::cg_event_tap_proxy::CGEventTapProxy;
use crate::cg_event_timestamp::CGEventTimestamp;
use crate::cg_event_type::CGEventType;
use crate::error::CGError;
use crate::event::{Event, Point};
use crate::ffi;

/// Notification name posted when an event tap is installed.
pub const EVENT_TAP_ADDED_NOTIFICATION: &str = "com.apple.coregraphics.eventTapAdded";
/// Notification name posted when an event tap is released.
pub const EVENT_TAP_REMOVED_NOTIFICATION: &str = "com.apple.coregraphics.eventTapRemoved";

/// Where a new event tap is inserted relative to existing taps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TapPlacement {
    #[default]
    HeadInsert,
    TailAppend,
}

impl TapPlacement {
    const fn raw(self) -> u32 {
        match self {
            Self::HeadInsert => 0,
            Self::TailAppend => 1,
        }
    }
}

/// What the tap callback wants to do with an intercepted event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TapAction {
    Pass,
    Drop,
}

/// A view into one intercepted event. Lives only for the duration of the callback.
pub struct TappedEvent<'a> {
    ptr: ffi::CGEventBridgeHandle,
    proxy: ffi::CGEventTapProxyBridgeHandle,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl TappedEvent<'_> {
    #[must_use]
    pub fn event_type(&self) -> u32 {
        unsafe { ffi::cg_event::cgevent_get_type(self.ptr) }
    }

    #[must_use]
    pub fn event_type_typed(&self) -> Option<CGEventType> {
        CGEventType::from_raw(self.event_type())
    }

    #[must_use]
    pub fn location(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cg_event::cgevent_get_location(self.ptr, &mut x, &mut y) };
        Point::new(x, y)
    }

    pub fn set_location(&self, location: Point) {
        unsafe { ffi::cg_event::cgevent_set_location(self.ptr, location.x, location.y) };
    }

    #[must_use]
    pub fn flags(&self) -> CGEventFlags {
        CGEventFlags::from_bits_truncate(unsafe { ffi::cg_event::cgevent_get_flags(self.ptr) })
    }

    pub fn set_flags(&self, flags: CGEventFlags) {
        unsafe { ffi::cg_event::cgevent_set_flags(self.ptr, flags.bits()) };
    }

    #[must_use]
    pub fn keycode(&self) -> u16 {
        let raw = unsafe {
            ffi::cg_event::cgevent_get_integer_value_field(self.ptr, CGEventField::KeyboardEventKeycode.raw())
        };
        u16::try_from(raw).unwrap_or(0)
    }

    #[must_use]
    pub fn integer_value(&self, field: CGEventField) -> i64 {
        unsafe { ffi::cg_event::cgevent_get_integer_value_field(self.ptr, field.raw()) }
    }

    pub fn set_integer_value(&self, field: CGEventField, value: i64) {
        unsafe { ffi::cg_event::cgevent_set_integer_value_field(self.ptr, field.raw(), value) };
    }

    #[must_use]
    pub fn double_value(&self, field: CGEventField) -> f64 {
        unsafe { ffi::cg_event::cgevent_get_double_value_field(self.ptr, field.raw()) }
    }

    pub fn set_double_value(&self, field: CGEventField, value: f64) {
        unsafe { ffi::cg_event::cgevent_set_double_value_field(self.ptr, field.raw(), value) };
    }

    #[must_use]
    pub fn timestamp(&self) -> u64 {
        unsafe { ffi::cg_event::cgevent_get_timestamp(self.ptr) }
    }

    #[must_use]
    pub fn event_timestamp(&self) -> CGEventTimestamp {
        CGEventTimestamp(self.timestamp())
    }

    pub fn set_timestamp(&self, timestamp: u64) {
        unsafe { ffi::cg_event::cgevent_set_timestamp(self.ptr, timestamp) };
    }

    pub fn set_event_timestamp(&self, timestamp: CGEventTimestamp) {
        self.set_timestamp(timestamp.raw());
    }

    #[must_use]
    pub fn unicode_string(&self) -> String {
        let len = unsafe { ffi::cg_event::cgevent_keyboard_get_unicode_string_length(self.ptr) };
        if len == 0 {
            return String::new();
        }
        let mut buf = vec![0_u16; len];
        let ok = unsafe {
            ffi::cg_event::cgevent_keyboard_get_unicode_string(self.ptr, buf.as_mut_ptr(), len)
        };
        if ok {
            String::from_utf16_lossy(&buf)
        } else {
            String::new()
        }
    }

    pub fn set_unicode_string(&self, string: &str) {
        let utf16: Vec<u16> = string.encode_utf16().collect();
        unsafe {
            ffi::cg_event::cgevent_keyboard_set_unicode_string(self.ptr, utf16.as_ptr(), utf16.len());
        };
    }

    #[must_use]
    pub fn mouse_subtype(&self) -> Option<CGEventMouseSubtype> {
        let raw = self.integer_value(CGEventField::MouseEventSubtype);
        u32::try_from(raw)
            .ok()
            .and_then(CGEventMouseSubtype::from_raw)
    }

    pub fn set_mouse_subtype(&self, subtype: CGEventMouseSubtype) {
        self.set_integer_value(CGEventField::MouseEventSubtype, i64::from(subtype.raw()));
    }

    #[must_use]
    pub const fn proxy(&self) -> CGEventTapProxy<'_> {
        CGEventTapProxy::from_raw(self.proxy)
    }

    /// Post a synthetic event back into the stream from this tap point.
    pub fn post(&self, event: &Event) {
        self.proxy().post_event(event);
    }
}

type Callback = Box<dyn FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static>;

struct TapInner {
    callback: Mutex<Callback>,
}

/// Snapshot of one installed event tap returned by [`EventTap::installed`].
#[derive(Debug, Clone)]
pub struct EventTapInformation {
    pub event_tap_id: u32,
    pub tap_point: CGEventTapLocation,
    pub options: CGEventTapOptions,
    pub events_of_interest: u64,
    pub tapping_process: i32,
    pub process_being_tapped: i32,
    pub enabled: bool,
    pub min_usec_latency: f32,
    pub avg_usec_latency: f32,
    pub max_usec_latency: f32,
}

impl From<ffi::CGEventTapInformation> for EventTapInformation {
    fn from(raw: ffi::CGEventTapInformation) -> Self {
        Self {
            event_tap_id: raw.event_tap_id,
            tap_point: CGEventTapLocation::from_raw(raw.tap_point).unwrap_or_default(),
            options: CGEventTapOptions::from_raw(raw.options).unwrap_or_default(),
            events_of_interest: raw.events_of_interest,
            tapping_process: raw.tapping_process,
            process_being_tapped: raw.process_being_tapped,
            enabled: raw.enabled,
            min_usec_latency: raw.min_usec_latency,
            avg_usec_latency: raw.avg_usec_latency,
            max_usec_latency: raw.max_usec_latency,
        }
    }
}

/// A live event tap. Drops the underlying mach port on scope exit.
pub struct EventTap {
    ptr: ffi::CGEventTapBridgeHandle,
    _inner: Box<TapInner>,
}

unsafe impl Send for EventTap {}
unsafe impl Sync for EventTap {}

impl Drop for EventTap {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::cg_event_tap::cgevent_tap_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn trampoline(
    context: *mut c_void,
    proxy: ffi::CGEventTapProxyBridgeHandle,
    _type: u32,
    event: *mut c_void,
) -> i32 {
    let inner: &TapInner = unsafe { &*context.cast::<TapInner>() };
    let tapped = TappedEvent {
        ptr: event,
        proxy,
        _phantom: core::marker::PhantomData,
    };
    let action = inner
        .callback
        .lock()
        .map_or(TapAction::Pass, |mut callback| callback(&tapped));
    match action {
        TapAction::Pass => 0,
        TapAction::Drop => 1,
    }
}

impl EventTap {
    /// Create a tap that observes / drops events of every type matching `events_mask`.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::TapCreateFailed`] when Apple refuses — typically missing Accessibility permission.
    pub fn new<F>(location: TapLocation, events_mask: u64, callback: F) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        Self::new_with_options(
            location,
            TapPlacement::HeadInsert,
            CGEventTapOptions::Default,
            events_mask,
            callback,
        )
    }

    /// Create a tap with explicit placement and options.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::TapCreateFailed`] when Apple refuses — typically missing Accessibility permission.
    pub fn new_with_options<F>(
        location: TapLocation,
        placement: TapPlacement,
        options: CGEventTapOptions,
        events_mask: u64,
        callback: F,
    ) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        let inner = Box::new(TapInner {
            callback: Mutex::new(Box::new(callback)),
        });
        let context = std::ptr::addr_of!(*inner).cast::<c_void>().cast_mut();
        let ptr = unsafe {
            ffi::cg_event_tap::cgevent_tap_create(
                location.raw(),
                placement.raw(),
                options.raw(),
                events_mask,
                trampoline,
                context,
            )
        };
        if ptr.is_null() {
            Err(CGError::TapCreateFailed)
        } else {
            Ok(Self { ptr, _inner: inner })
        }
    }

    /// Create a per-process tap using `CGEventTapCreateForPid`.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::TapCreateFailed`] when Apple refuses — typically missing Accessibility permission.
    pub fn for_pid<F>(
        pid: i32,
        placement: TapPlacement,
        options: CGEventTapOptions,
        events_mask: u64,
        callback: F,
    ) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        let inner = Box::new(TapInner {
            callback: Mutex::new(Box::new(callback)),
        });
        let context = std::ptr::addr_of!(*inner).cast::<c_void>().cast_mut();
        let ptr = unsafe {
            ffi::cg_event_tap::cgevent_tap_create_for_pid(
                pid,
                placement.raw(),
                options.raw(),
                events_mask,
                trampoline,
                context,
            )
        };
        if ptr.is_null() {
            Err(CGError::TapCreateFailed)
        } else {
            Ok(Self { ptr, _inner: inner })
        }
    }

    /// Convenience constructor: tap every keyboard event.
    ///
    /// # Errors
    ///
    /// See [`Self::new`].
    pub fn keyboard<F>(callback: F) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        let mask = CGEventType::KeyDown.mask_bit()
            | CGEventType::KeyUp.mask_bit()
            | CGEventType::FlagsChanged.mask_bit();
        Self::new(TapLocation::Session, mask, callback)
    }

    /// Convenience constructor: tap every mouse event.
    ///
    /// # Errors
    ///
    /// See [`Self::new`].
    pub fn mouse<F>(callback: F) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        let mask = CGEventType::MouseMoved.mask_bit()
            | CGEventType::LeftMouseDown.mask_bit()
            | CGEventType::LeftMouseUp.mask_bit()
            | CGEventType::RightMouseDown.mask_bit()
            | CGEventType::RightMouseUp.mask_bit()
            | CGEventType::LeftMouseDragged.mask_bit()
            | CGEventType::RightMouseDragged.mask_bit()
            | CGEventType::ScrollWheel.mask_bit();
        Self::new(TapLocation::Session, mask, callback)
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::cg_event_tap::cgevent_tap_is_enabled(self.ptr) }
    }

    pub fn enable(&self) {
        unsafe { ffi::cg_event_tap::cgevent_tap_enable(self.ptr, true) };
    }

    pub fn disable(&self) {
        unsafe { ffi::cg_event_tap::cgevent_tap_enable(self.ptr, false) };
    }

    /// Stop this tap's run loop from any thread.
    pub fn stop(&self) {
        unsafe { ffi::cg_event_tap::cgevent_tap_stop(self.ptr) };
    }

    /// Run the current thread's run loop forever. Blocks.
    pub fn run(&self) {
        unsafe { ffi::cg_event_tap::cgevent_tap_run_current_run_loop() };
    }

    /// Stop a running run loop. Call from the same thread or another callback-triggered context.
    pub fn stop_current_run_loop() {
        unsafe { ffi::cg_event_tap::cgevent_tap_stop_current_run_loop() };
    }

    #[must_use]
    pub fn preflight_listen_access() -> bool {
        unsafe { ffi::cg_event_tap::cgevent_preflight_listen_event_access() }
    }

    #[must_use]
    pub fn request_listen_access() -> bool {
        unsafe { ffi::cg_event_tap::cgevent_request_listen_event_access() }
    }

    #[must_use]
    pub fn preflight_post_access() -> bool {
        unsafe { ffi::cg_event_tap::cgevent_preflight_post_event_access() }
    }

    #[must_use]
    pub fn request_post_access() -> bool {
        unsafe { ffi::cg_event_tap::cgevent_request_post_event_access() }
    }

    /// Return a snapshot of the currently installed taps via `CGGetEventTapList`.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::CoreGraphicsError`] if `CGGetEventTapList` returns a non-zero `CGError`.
    pub fn installed() -> Result<Vec<EventTapInformation>, CGError> {
        let mut count = 0_u32;
        let code = unsafe { ffi::cg_event_tap::cgevent_get_event_tap_list(0, ptr::null_mut(), &mut count) };
        if code != 0 {
            return Err(CGError::CoreGraphicsError {
                operation: "CGGetEventTapList",
                code,
            });
        }
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut raw = vec![ffi::CGEventTapInformation::default(); count as usize];
        let code = unsafe {
            ffi::cg_event_tap::cgevent_get_event_tap_list(count, raw.as_mut_ptr().cast(), &mut count)
        };
        if code != 0 {
            return Err(CGError::CoreGraphicsError {
                operation: "CGGetEventTapList",
                code,
            });
        }
        raw.truncate(count as usize);
        Ok(raw.into_iter().map(EventTapInformation::from).collect())
    }
}
