//! `CGEventTapProxy` wrapper exposed during tap callbacks.

use core::{ffi::c_void, fmt, marker::PhantomData};

use crate::{event::Event, ffi};

#[derive(Clone, Copy)]
pub struct CGEventTapProxy<'a> {
    raw: *mut c_void,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for CGEventTapProxy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CGEventTapProxy")
            .field("raw", &self.raw)
            .finish()
    }
}

impl CGEventTapProxy<'_> {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_raw(self) -> *mut c_void {
        self.raw
    }

    #[must_use]
    pub fn is_null(self) -> bool {
        self.raw.is_null()
    }

    /// Post a synthetic event back into the stream at this tap point.
    pub fn post_event(self, event: &Event) {
        unsafe { ffi::cg_event_tap_proxy::cgevent_tap_proxy_post_event(self.raw, event.ptr) };
    }
}
