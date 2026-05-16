//! `CGEventSource` — represents the origin of a synthesised event.

use core::ptr;

use crate::error::CGError;
use crate::event::{ModifierFlags, MouseButton};
use crate::ffi;

bitflags::bitflags! {
    /// Local hardware event classes permitted during suppression windows.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LocalEventsFilter: u32 {
        const LOCAL_MOUSE = ffi::kCGEventFilterMaskPermitLocalMouseEvents;
        const LOCAL_KEYBOARD = ffi::kCGEventFilterMaskPermitLocalKeyboardEvents;
        const SYSTEM_DEFINED = ffi::kCGEventFilterMaskPermitSystemDefinedEvents;
        const ALL = ffi::kCGEventFilterMaskPermitAllEvents;
    }
}

/// Origin of a synthesised event. Mirrors `CGEventSourceStateID`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceState {
    Private,
    CombinedSession,
    HIDSystem,
}

impl SourceState {
    pub(crate) const fn as_raw(self) -> ffi::CGEventSourceStateID {
        match self {
            Self::Private => ffi::kCGEventSourceStatePrivate,
            Self::CombinedSession => ffi::kCGEventSourceStateCombinedSessionState,
            Self::HIDSystem => ffi::kCGEventSourceStateHIDSystemState,
        }
    }
}

/// Event-suppression phases used by `CGEventSource` local event filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuppressionState {
    Interval,
    RemoteMouseDrag,
}

impl SuppressionState {
    pub(crate) const fn as_raw(self) -> ffi::CGEventSuppressionState {
        match self {
            Self::Interval => ffi::kCGEventSuppressionStateSuppressionInterval,
            Self::RemoteMouseDrag => ffi::kCGEventSuppressionStateRemoteMouseDrag,
        }
    }
}

/// A retained `CGEventSource`.
pub struct EventSource {
    pub(crate) ptr: ffi::CGEventSourceRef,
}

unsafe impl Send for EventSource {}
unsafe impl Sync for EventSource {}

impl EventSource {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::CGEventSourceGetTypeID() }
    }

    /// Create a new event source with the requested state.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::SourceCreateFailed`] if Apple returns NULL.
    pub fn new(state: SourceState) -> Result<Self, CGError> {
        let ptr = unsafe { ffi::CGEventSourceCreate(state.as_raw()) };
        if ptr.is_null() {
            Err(CGError::SourceCreateFailed)
        } else {
            Ok(Self { ptr })
        }
    }

    /// The default source — `Private` state, suitable for most automation.
    ///
    /// # Errors
    ///
    /// See [`Self::new`].
    pub fn private() -> Result<Self, CGError> {
        Self::new(SourceState::Private)
    }

    #[must_use]
    pub fn keyboard_type(&self) -> ffi::CGEventSourceKeyboardType {
        unsafe { ffi::CGEventSourceGetKeyboardType(self.ptr) }
    }

    pub fn set_keyboard_type(&self, keyboard_type: ffi::CGEventSourceKeyboardType) {
        unsafe { ffi::CGEventSourceSetKeyboardType(self.ptr, keyboard_type) };
    }

    #[must_use]
    pub fn pixels_per_line(&self) -> f64 {
        unsafe { ffi::CGEventSourceGetPixelsPerLine(self.ptr) }
    }

    pub fn set_pixels_per_line(&self, pixels_per_line: f64) {
        unsafe { ffi::CGEventSourceSetPixelsPerLine(self.ptr, pixels_per_line) };
    }

    #[must_use]
    pub fn source_state_id(&self) -> ffi::CGEventSourceStateID {
        unsafe { ffi::CGEventSourceGetSourceStateID(self.ptr) }
    }

    #[must_use]
    pub fn button_state(&self, button: MouseButton) -> bool {
        unsafe { ffi::CGEventSourceButtonState(self.source_state_id(), button.as_raw()) }
    }

    #[must_use]
    pub fn key_state(&self, keycode: u16) -> bool {
        unsafe { ffi::CGEventSourceKeyState(self.source_state_id(), keycode) }
    }

    #[must_use]
    pub fn flags_state(&self) -> ModifierFlags {
        ModifierFlags::from_bits_truncate(unsafe {
            ffi::CGEventSourceFlagsState(self.source_state_id())
        })
    }

    #[must_use]
    pub fn seconds_since_last_event_type(&self, event_type: ffi::CGEventType) -> f64 {
        unsafe { ffi::CGEventSourceSecondsSinceLastEventType(self.source_state_id(), event_type) }
    }

    #[must_use]
    pub fn seconds_since_last_input_event(&self) -> f64 {
        self.seconds_since_last_event_type(ffi::kCGAnyInputEventType)
    }

    #[must_use]
    pub fn counter_for_event_type(&self, event_type: ffi::CGEventType) -> u32 {
        unsafe { ffi::CGEventSourceCounterForEventType(self.source_state_id(), event_type) }
    }

    #[must_use]
    pub fn user_data(&self) -> i64 {
        unsafe { ffi::CGEventSourceGetUserData(self.ptr) }
    }

    pub fn set_user_data(&self, user_data: i64) {
        unsafe { ffi::CGEventSourceSetUserData(self.ptr, user_data) };
    }

    pub fn set_local_events_filter(&self, filter: LocalEventsFilter, state: SuppressionState) {
        unsafe {
            ffi::CGEventSourceSetLocalEventsFilterDuringSuppressionState(
                self.ptr,
                filter.bits(),
                state.as_raw(),
            );
        };
    }

    #[must_use]
    pub fn local_events_filter(&self, state: SuppressionState) -> LocalEventsFilter {
        LocalEventsFilter::from_bits_truncate(unsafe {
            ffi::CGEventSourceGetLocalEventsFilterDuringSuppressionState(self.ptr, state.as_raw())
        })
    }

    pub fn set_local_events_suppression_interval(&self, seconds: f64) {
        unsafe { ffi::CGEventSourceSetLocalEventsSuppressionInterval(self.ptr, seconds) };
    }

    #[must_use]
    pub fn local_events_suppression_interval(&self) -> f64 {
        unsafe { ffi::CGEventSourceGetLocalEventsSuppressionInterval(self.ptr) }
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::CFRelease(self.ptr.cast_const()) };
            self.ptr = ptr::null_mut();
        }
    }
}
