//! `CGEventSource` — represents the origin of a synthesised event.

use core::ptr;

use crate::cg_event_flags::CGEventFlags;
use crate::cg_event_type::{CGEventType, CG_ANY_INPUT_EVENT_TYPE};
use crate::error::CGError;
use crate::event::MouseButton;
use crate::ffi;

bitflags::bitflags! {
    /// Local hardware event classes permitted during suppression windows.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LocalEventsFilter: u32 {
        const LOCAL_MOUSE = 0x0000_0001;
        const LOCAL_KEYBOARD = 0x0000_0002;
        const SYSTEM_DEFINED = 0x0000_0004;
        const ALL = 0x0000_0007;
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
    pub(crate) const fn as_raw(self) -> i32 {
        match self {
            Self::Private => -1,
            Self::CombinedSession => 0,
            Self::HIDSystem => 1,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            -1 => Some(Self::Private),
            0 => Some(Self::CombinedSession),
            1 => Some(Self::HIDSystem),
            _ => None,
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
    pub(crate) const fn as_raw(self) -> u32 {
        match self {
            Self::Interval => 0,
            Self::RemoteMouseDrag => 1,
        }
    }
}

/// A retained `CGEventSource`.
pub struct EventSource {
    pub(crate) ptr: ffi::CGEventSourceBridgeHandle,
}

unsafe impl Send for EventSource {}
unsafe impl Sync for EventSource {}

impl EventSource {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { ffi::cg_event_source::cgevent_source_get_type_id() }
    }

    /// Create a new event source with the requested state.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::SourceCreateFailed`] if Apple returns NULL.
    pub fn new(state: SourceState) -> Result<Self, CGError> {
        let ptr = unsafe { ffi::cg_event_source::cgevent_source_create(state.as_raw()) };
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
    pub fn keyboard_type(&self) -> u32 {
        unsafe { ffi::cg_event_source::cgevent_source_get_keyboard_type(self.ptr) }
    }

    pub fn set_keyboard_type(&self, keyboard_type: u32) {
        unsafe { ffi::cg_event_source::cgevent_source_set_keyboard_type(self.ptr, keyboard_type) };
    }

    #[must_use]
    pub fn pixels_per_line(&self) -> f64 {
        unsafe { ffi::cg_event_source::cgevent_source_get_pixels_per_line(self.ptr) }
    }

    pub fn set_pixels_per_line(&self, pixels_per_line: f64) {
        unsafe {
            ffi::cg_event_source::cgevent_source_set_pixels_per_line(self.ptr, pixels_per_line);
        };
    }

    #[must_use]
    pub fn source_state_id(&self) -> i32 {
        unsafe { ffi::cg_event_source::cgevent_source_get_source_state_id(self.ptr) }
    }

    #[must_use]
    pub fn source_state(&self) -> Option<SourceState> {
        SourceState::from_raw(self.source_state_id())
    }

    #[must_use]
    pub fn button_state(&self, button: MouseButton) -> bool {
        unsafe { ffi::cg_event_source::cgevent_source_button_state(self.source_state_id(), button.as_raw()) }
    }

    #[must_use]
    pub fn key_state(&self, keycode: u16) -> bool {
        unsafe { ffi::cg_event_source::cgevent_source_key_state(self.source_state_id(), keycode) }
    }

    #[must_use]
    pub fn flags_state(&self) -> CGEventFlags {
        CGEventFlags::from_bits_truncate(unsafe {
            ffi::cg_event_source::cgevent_source_flags_state(self.source_state_id())
        })
    }

    #[must_use]
    pub fn seconds_since_last_event_type(&self, event_type: u32) -> f64 {
        unsafe {
            ffi::cg_event_source::cgevent_source_seconds_since_last_event_type(
                self.source_state_id(),
                event_type,
            )
        }
    }

    #[must_use]
    pub fn seconds_since_last_typed_event(&self, event_type: CGEventType) -> f64 {
        self.seconds_since_last_event_type(event_type.raw())
    }

    #[must_use]
    pub fn seconds_since_last_input_event(&self) -> f64 {
        self.seconds_since_last_event_type(CG_ANY_INPUT_EVENT_TYPE)
    }

    #[must_use]
    pub fn counter_for_event_type(&self, event_type: u32) -> u32 {
        unsafe {
            ffi::cg_event_source::cgevent_source_counter_for_event_type(
                self.source_state_id(),
                event_type,
            )
        }
    }

    #[must_use]
    pub fn counter_for_typed_event(&self, event_type: CGEventType) -> u32 {
        self.counter_for_event_type(event_type.raw())
    }

    #[must_use]
    pub fn user_data(&self) -> i64 {
        unsafe { ffi::cg_event_source::cgevent_source_get_user_data(self.ptr) }
    }

    pub fn set_user_data(&self, user_data: i64) {
        unsafe { ffi::cg_event_source::cgevent_source_set_user_data(self.ptr, user_data) };
    }

    pub fn set_local_events_filter(&self, filter: LocalEventsFilter, state: SuppressionState) {
        unsafe {
            ffi::cg_event_source::cgevent_source_set_local_events_filter_during_suppression_state(
                self.ptr,
                filter.bits(),
                state.as_raw(),
            );
        };
    }

    #[must_use]
    pub fn local_events_filter(&self, state: SuppressionState) -> LocalEventsFilter {
        LocalEventsFilter::from_bits_truncate(unsafe {
            ffi::cg_event_source::cgevent_source_get_local_events_filter_during_suppression_state(
                self.ptr,
                state.as_raw(),
            )
        })
    }

    pub fn set_local_events_suppression_interval(&self, seconds: f64) {
        unsafe {
            ffi::cg_event_source::cgevent_source_set_local_events_suppression_interval(
                self.ptr,
                seconds,
            );
        };
    }

    #[must_use]
    pub fn local_events_suppression_interval(&self) -> f64 {
        unsafe { ffi::cg_event_source::cgevent_source_get_local_events_suppression_interval(self.ptr) }
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::cg_event_source::cgevent_source_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}
