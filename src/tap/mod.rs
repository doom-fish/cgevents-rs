//! `EventTap` — intercept (and optionally modify or drop) events as they
//! flow through the system.

use core::ffi::c_void;
use core::ptr;
use std::sync::Mutex;

use crate::error::CGError;
use crate::event::{Event, ModifierFlags, Point, TapLocation};
use crate::ffi;

/// What the tap callback wants to do with an intercepted event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TapAction {
    Pass,
    Drop,
}

/// A view into one intercepted event. Lives only for the duration of
/// the callback.
pub struct TappedEvent<'a> {
    ptr: ffi::CGEventRef,
    proxy: ffi::CGEventTapProxy,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl TappedEvent<'_> {
    const fn view(&self) -> ManualDropEvent {
        ManualDropEvent { ptr: self.ptr }
    }

    #[must_use]
    pub fn event_type(&self) -> ffi::CGEventType {
        self.view().event_type()
    }

    #[must_use]
    pub fn location(&self) -> Point {
        self.view().location()
    }

    #[must_use]
    pub fn flags(&self) -> ModifierFlags {
        self.view().flags()
    }

    #[must_use]
    pub fn keycode(&self) -> u16 {
        self.view().keycode()
    }

    /// Post a synthetic event back into the stream from this tap point.
    pub fn post(&self, event: &Event) {
        unsafe { ffi::CGEventTapPostEvent(self.proxy, event.ptr) };
    }
}

struct ManualDropEvent {
    ptr: ffi::CGEventRef,
}

impl ManualDropEvent {
    fn event_type(&self) -> ffi::CGEventType {
        unsafe { ffi::CGEventGetType(self.ptr) }
    }
    fn location(&self) -> Point {
        Point::from(unsafe { ffi::CGEventGetLocation(self.ptr) })
    }
    fn flags(&self) -> ModifierFlags {
        ModifierFlags::from_bits_truncate(unsafe { ffi::CGEventGetFlags(self.ptr) })
    }
    fn keycode(&self) -> u16 {
        let v = unsafe { ffi::CGEventGetIntegerValueField(self.ptr, ffi::kCGKeyboardEventKeycode) };
        u16::try_from(v).unwrap_or(0)
    }
}

type Callback = Box<dyn FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static>;

struct TapInner {
    callback: Mutex<Callback>,
}

/// A live event tap. Drops the underlying mach port on scope exit.
pub struct EventTap {
    port: ffi::CFMachPortRef,
    run_loop_source: ffi::CFRunLoopSourceRef,
    _inner: Box<TapInner>,
}

unsafe impl Send for EventTap {}

impl Drop for EventTap {
    fn drop(&mut self) {
        unsafe {
            if !self.run_loop_source.is_null() {
                ffi::CFRunLoopRemoveSource(
                    ffi::CFRunLoopGetCurrent(),
                    self.run_loop_source,
                    ffi::kCFRunLoopCommonModes,
                );
                ffi::CFRelease(self.run_loop_source.cast_const());
                self.run_loop_source = ptr::null_mut();
            }
            if !self.port.is_null() {
                ffi::CFMachPortInvalidate(self.port);
                ffi::CFRelease(self.port.cast_const());
                self.port = ptr::null_mut();
            }
        }
    }
}

unsafe extern "C" fn trampoline(
    proxy: ffi::CGEventTapProxy,
    _ty: ffi::CGEventType,
    event: ffi::CGEventRef,
    user_info: *mut c_void,
) -> ffi::CGEventRef {
    let inner: &TapInner = unsafe { &*user_info.cast::<TapInner>() };
    let tapped = TappedEvent {
        ptr: event,
        proxy,
        _phantom: core::marker::PhantomData,
    };
    let action = inner
        .callback
        .lock()
        .map_or(TapAction::Pass, |mut cb| cb(&tapped));
    match action {
        TapAction::Pass => event,
        TapAction::Drop => ptr::null_mut(),
    }
}

impl EventTap {
    /// Create a tap that observes / drops events of every type matching
    /// `events_mask`.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::TapCreateFailed`] when Apple refuses — typically
    /// missing Accessibility permission.
    pub fn new<F>(location: TapLocation, events_mask: u64, callback: F) -> Result<Self, CGError>
    where
        F: FnMut(&TappedEvent<'_>) -> TapAction + Send + 'static,
    {
        let inner = Box::new(TapInner {
            callback: Mutex::new(Box::new(callback)),
        });
        let user_info = std::ptr::addr_of!(*inner).cast::<c_void>().cast_mut();

        let port = unsafe {
            ffi::CGEventTapCreate(
                location.as_raw(),
                ffi::kCGHeadInsertEventTap,
                ffi::kCGEventTapOptionDefault,
                events_mask,
                trampoline,
                user_info,
            )
        };
        if port.is_null() {
            return Err(CGError::TapCreateFailed);
        }

        let run_loop_source =
            unsafe { ffi::CFMachPortCreateRunLoopSource(ffi::kCFAllocatorDefault, port, 0) };
        if run_loop_source.is_null() {
            unsafe {
                ffi::CFMachPortInvalidate(port);
                ffi::CFRelease(port.cast_const());
            }
            return Err(CGError::TapCreateFailed);
        }

        unsafe {
            ffi::CFRunLoopAddSource(
                ffi::CFRunLoopGetCurrent(),
                run_loop_source,
                ffi::kCFRunLoopCommonModes,
            );
            ffi::CGEventTapEnable(port, true);
        }

        Ok(Self {
            port,
            run_loop_source,
            _inner: inner,
        })
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
        let mask = ffi::cg_event_mask_bit(ffi::kCGEventKeyDown)
            | ffi::cg_event_mask_bit(ffi::kCGEventKeyUp)
            | ffi::cg_event_mask_bit(ffi::kCGEventFlagsChanged);
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
        let mask = ffi::cg_event_mask_bit(ffi::kCGEventMouseMoved)
            | ffi::cg_event_mask_bit(ffi::kCGEventLeftMouseDown)
            | ffi::cg_event_mask_bit(ffi::kCGEventLeftMouseUp)
            | ffi::cg_event_mask_bit(ffi::kCGEventRightMouseDown)
            | ffi::cg_event_mask_bit(ffi::kCGEventRightMouseUp)
            | ffi::cg_event_mask_bit(ffi::kCGEventLeftMouseDragged)
            | ffi::cg_event_mask_bit(ffi::kCGEventRightMouseDragged)
            | ffi::cg_event_mask_bit(ffi::kCGEventScrollWheel);
        Self::new(TapLocation::Session, mask, callback)
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::CGEventTapIsEnabled(self.port) }
    }

    /// Run the current thread's run loop forever. Blocks.
    pub fn run(&self) {
        unsafe { ffi::CFRunLoopRun() };
    }

    /// Stop a running run loop. Call from another thread.
    pub fn stop_current_run_loop() {
        unsafe { ffi::CFRunLoopStop(ffi::CFRunLoopGetCurrent()) };
    }
}
