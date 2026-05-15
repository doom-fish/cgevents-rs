//! `CGEventSource` — represents the origin of a synthesised event.

use core::ptr;

use crate::error::CGError;
use crate::ffi;

/// Origin of a synthesised event. Mirrors `CGEventSourceStateID`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceState {
    /// Independent of the user's HID + system state. Good default for
    /// most automation: posted events don't fight with currently-pressed
    /// physical keys.
    Private,
    /// Combined session state — events look like they came from a real
    /// user action.
    CombinedSession,
    /// Event-tap-style HID system state.
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

/// A retained `CGEventSource`.
pub struct EventSource {
    pub(crate) ptr: ffi::CGEventSourceRef,
}

unsafe impl Send for EventSource {}
unsafe impl Sync for EventSource {}

impl EventSource {
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
}

impl Drop for EventSource {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::CFRelease(self.ptr.cast_const()) };
            self.ptr = ptr::null_mut();
        }
    }
}
