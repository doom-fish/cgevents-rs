//! `CGEventTap` — intercept (and optionally modify or drop) events as they flow through the system.

use core::ffi::c_void;
use core::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError, TryLockError};

use doom_fish_utils::callback_context::CallbackContext;

use crate::cg_event_field::CGEventField;
use crate::cg_event_flags::CGEventFlags;
use crate::cg_event_mouse_subtype::CGEventMouseSubtype;
use crate::cg_event_tap_location::{CGEventTapLocation, TapLocation};
use crate::cg_event_tap_options::CGEventTapOptions;
use crate::cg_event_tap_proxy::CGEventTapProxy;
use crate::cg_event_timestamp::CGEventTimestamp;
use crate::cg_event_type::CGEventType;
use crate::cg_momentum_scroll_phase::CGMomentumScrollPhase;
use crate::cg_scroll_phase::CGScrollPhase;
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
#[derive(Debug)]
#[non_exhaustive]
pub enum TapAction {
    Pass,
    Drop,
    Replace(Event),
}

/// A view into one intercepted event. Lives only for the duration of the callback.
pub struct TappedEvent<'a> {
    ptr: ffi::CGEventBridgeHandle,
    proxy: ffi::CGEventTapProxyBridgeHandle,
    event_type: u32,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl TappedEvent<'_> {
    #[must_use]
    pub const fn event_type(&self) -> u32 {
        self.event_type
    }

    #[must_use]
    pub const fn event_type_typed(&self) -> Option<CGEventType> {
        CGEventType::from_raw(self.event_type())
    }

    #[must_use]
    pub fn location(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cg_event::cgevent_get_location(self.ptr, &raw mut x, &raw mut y) };
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
            ffi::cg_event::cgevent_get_integer_value_field(
                self.ptr,
                CGEventField::KeyboardEventKeycode.raw(),
            )
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

    #[allow(clippy::missing_errors_doc)]
    pub fn set_unicode_string(&self, string: &str) -> Result<(), CGError> {
        let utf16 = crate::event::keyboard_unicode_units(string)?;
        unsafe {
            ffi::cg_event::cgevent_keyboard_set_unicode_string(
                self.ptr,
                utf16.as_ptr(),
                utf16.len(),
            );
        };
        Ok(())
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
    pub fn scroll_phase(&self) -> Option<CGScrollPhase> {
        let raw = self.integer_value(CGEventField::ScrollWheelEventScrollPhase);
        u32::try_from(raw).ok().and_then(CGScrollPhase::from_raw)
    }

    pub fn set_scroll_phase(&self, phase: CGScrollPhase) {
        self.set_integer_value(
            CGEventField::ScrollWheelEventScrollPhase,
            i64::from(phase.raw()),
        );
    }

    #[must_use]
    pub fn momentum_scroll_phase(&self) -> Option<CGMomentumScrollPhase> {
        let raw = self.integer_value(CGEventField::ScrollWheelEventMomentumPhase);
        u32::try_from(raw)
            .ok()
            .and_then(CGMomentumScrollPhase::from_raw)
    }

    pub fn set_momentum_scroll_phase(&self, phase: CGMomentumScrollPhase) {
        self.set_integer_value(
            CGEventField::ScrollWheelEventMomentumPhase,
            i64::from(phase.raw()),
        );
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

const TAP_DISABLED_BY_TIMEOUT: u32 = 0xFFFF_FFFE;
const TAP_DISABLED_BY_USER_INPUT: u32 = 0xFFFF_FFFF;

const TAP_PASS: i32 = 0;
const TAP_DROP: i32 = 1;
const TAP_REPLACE: i32 = 2;
const TAP_REENABLE: i32 = 3;

struct TapState {
    callback: Mutex<Callback>,
    auto_reenable: AtomicBool,
}

type TapContext = CallbackContext<TapState>;

fn new_tap_context(callback: Callback) -> TapContext {
    TapContext::new(TapState {
        callback: Mutex::new(callback),
        auto_reenable: AtomicBool::new(true),
    })
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
    context: TapContext,
}

unsafe impl Send for EventTap {}
unsafe impl Sync for EventTap {}

impl Drop for EventTap {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.ptr.is_null() {
            unsafe { ffi::cg_event_tap::cgevent_tap_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn trampoline(
    context: *mut c_void,
    proxy: ffi::CGEventTapProxyBridgeHandle,
    event_type: u32,
    event: *mut c_void,
    replacement: *mut *mut c_void,
) -> i32 {
    let tapped = TappedEvent {
        ptr: event,
        proxy,
        event_type,
        _phantom: core::marker::PhantomData,
    };
    let outcome = unsafe {
        TapContext::with(context, "EventTap callback", |state| {
            let action = match state.callback.try_lock() {
                Ok(mut callback) => callback(&tapped),
                Err(TryLockError::Poisoned(poisoned)) => {
                    let mut callback = PoisonError::into_inner(poisoned);
                    callback(&tapped)
                }
                Err(TryLockError::WouldBlock) => TapAction::Pass,
            };
            (action, state.auto_reenable.load(Ordering::Relaxed))
        })
    };
    let Some((action, auto_reenable)) = outcome else {
        return TAP_PASS;
    };
    if event_type == TAP_DISABLED_BY_TIMEOUT || event_type == TAP_DISABLED_BY_USER_INPUT {
        return if auto_reenable {
            TAP_REENABLE
        } else {
            TAP_PASS
        };
    }
    match action {
        TapAction::Pass => TAP_PASS,
        TapAction::Drop => TAP_DROP,
        TapAction::Replace(event) => {
            if replacement.is_null() {
                return TAP_PASS;
            }
            unsafe { *replacement = event.into_raw() };
            TAP_REPLACE
        }
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
        let context = new_tap_context(Box::new(callback));
        let ptr = unsafe {
            ffi::cg_event_tap::cgevent_tap_create(
                location.raw(),
                placement.raw(),
                options.raw(),
                events_mask,
                trampoline,
                context.as_ptr(),
                TapContext::RETAIN,
                TapContext::RELEASE,
            )
        };
        if ptr.is_null() {
            Err(CGError::TapCreateFailed)
        } else {
            Ok(Self { ptr, context })
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
        let context = new_tap_context(Box::new(callback));
        let ptr = unsafe {
            ffi::cg_event_tap::cgevent_tap_create_for_pid(
                pid,
                placement.raw(),
                options.raw(),
                events_mask,
                trampoline,
                context.as_ptr(),
                TapContext::RETAIN,
                TapContext::RELEASE,
            )
        };
        if ptr.is_null() {
            Err(CGError::TapCreateFailed)
        } else {
            Ok(Self { ptr, context })
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

    #[must_use]
    pub fn auto_reenable(&self) -> bool {
        self.context.get().auto_reenable.load(Ordering::Relaxed)
    }

    pub fn set_auto_reenable(&self, enabled: bool) {
        self.context
            .get()
            .auto_reenable
            .store(enabled, Ordering::Relaxed);
    }

    /// Stop this tap's run loop from any thread.
    pub fn stop(&self) {
        unsafe { ffi::cg_event_tap::cgevent_tap_stop(self.ptr) };
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn run(&self) -> Result<(), CGError> {
        if unsafe { ffi::cg_event_tap::cgevent_tap_run(self.ptr) } {
            Ok(())
        } else {
            Err(CGError::WrongThread)
        }
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
        let code = unsafe {
            ffi::cg_event_tap::cgevent_get_event_tap_list(0, ptr::null_mut(), &raw mut count)
        };
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
            ffi::cg_event_tap::cgevent_get_event_tap_list(
                count,
                raw.as_mut_ptr().cast(),
                &raw mut count,
            )
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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::{
        new_tap_context, trampoline, TapAction, TAP_DISABLED_BY_TIMEOUT,
        TAP_DISABLED_BY_USER_INPUT, TAP_DROP, TAP_PASS, TAP_REENABLE, TAP_REPLACE,
    };
    use crate::cg_event_type::CGEventType;
    use crate::event::{Event, KeyEvent};
    use crate::source::EventSource;

    fn deliver(
        context: *mut core::ffi::c_void,
        event: &Event,
        event_type: u32,
    ) -> (i32, Option<Event>) {
        let mut replacement = core::ptr::null_mut();
        let code = unsafe {
            trampoline(
                context,
                core::ptr::null_mut(),
                event_type,
                event.ptr,
                &raw mut replacement,
            )
        };
        let replacement = (!replacement.is_null()).then(|| Event { ptr: replacement });
        (code, replacement)
    }

    #[test]
    fn trampoline_maps_every_action() {
        let source = EventSource::private().expect("event source");
        let event = KeyEvent::down(0).build(&source).expect("key event");
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        let context = new_tap_context(Box::new(move |_| {
            match counter.fetch_add(1, Ordering::SeqCst) {
                0 => TapAction::Pass,
                1 => TapAction::Drop,
                _ => {
                    let source = EventSource::private().expect("event source");
                    TapAction::Replace(KeyEvent::up(0).build(&source).expect("replacement"))
                }
            }
        }));

        let key_down = CGEventType::KeyDown.raw();
        assert_eq!(deliver(context.as_ptr(), &event, key_down).0, TAP_PASS);
        assert_eq!(deliver(context.as_ptr(), &event, key_down).0, TAP_DROP);
        let (code, replacement) = deliver(context.as_ptr(), &event, key_down);
        assert_eq!(code, TAP_REPLACE);
        let replacement = replacement.expect("replacement event handed to the bridge");
        assert_eq!(replacement.event_type_typed(), Some(CGEventType::KeyUp));
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn disabled_notifications_carry_the_real_type_and_request_reenable() {
        let event = Event::new(None).expect("event");
        let seen = Arc::new(AtomicU32::new(0));
        let sink = Arc::clone(&seen);
        let context = new_tap_context(Box::new(move |tapped| {
            sink.store(tapped.event_type(), Ordering::SeqCst);
            TapAction::Drop
        }));

        assert_eq!(
            deliver(context.as_ptr(), &event, TAP_DISABLED_BY_TIMEOUT).0,
            TAP_REENABLE
        );
        assert_eq!(seen.load(Ordering::SeqCst), TAP_DISABLED_BY_TIMEOUT);
        assert_ne!(event.event_type(), TAP_DISABLED_BY_TIMEOUT);

        context.get().auto_reenable.store(false, Ordering::SeqCst);
        assert_eq!(
            deliver(context.as_ptr(), &event, TAP_DISABLED_BY_USER_INPUT).0,
            TAP_PASS
        );
        assert_eq!(seen.load(Ordering::SeqCst), TAP_DISABLED_BY_USER_INPUT);
    }

    #[test]
    fn inactive_or_panicking_callbacks_pass_events_through() {
        let event = Event::new(None).expect("event");
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        let context = new_tap_context(Box::new(move |_| {
            assert_ne!(
                counter.fetch_add(1, Ordering::SeqCst),
                0,
                "tap callback panic"
            );
            TapAction::Drop
        }));
        let key_down = CGEventType::KeyDown.raw();

        assert_eq!(deliver(context.as_ptr(), &event, key_down).0, TAP_PASS);
        assert_eq!(deliver(context.as_ptr(), &event, key_down).0, TAP_DROP);
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        context.deactivate();
        assert_eq!(deliver(context.as_ptr(), &event, key_down).0, TAP_PASS);
        assert_eq!(deliver(core::ptr::null_mut(), &event, key_down).0, TAP_PASS);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }
}
