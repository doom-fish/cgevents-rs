//! High-level `Event` wrapper + `KeyEvent`, `MouseEvent`, `ScrollEvent`
//! builders that synthesise + post events.

use core::ptr;

use crate::error::CGError;
use crate::ffi;
use crate::source::EventSource;

bitflags::bitflags! {
    /// Modifier-key flags applied to keyboard / mouse events. Mirrors
    /// `CGEventFlags`.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct ModifierFlags: u64 {
        const ALPHA_SHIFT = ffi::kCGEventFlagMaskAlphaShift;
        const SHIFT       = ffi::kCGEventFlagMaskShift;
        const CONTROL     = ffi::kCGEventFlagMaskControl;
        const ALT         = ffi::kCGEventFlagMaskAlternate;
        const COMMAND     = ffi::kCGEventFlagMaskCommand;
        const HELP        = ffi::kCGEventFlagMaskHelp;
        const SECONDARY_FN = ffi::kCGEventFlagMaskSecondaryFn;
        const NUMERIC_PAD = ffi::kCGEventFlagMaskNumericPad;
    }
}

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Center,
    Other(u32),
}

impl MouseButton {
    pub(crate) const fn as_raw(self) -> ffi::CGMouseButton {
        match self {
            Self::Left => ffi::kCGMouseButtonLeft,
            Self::Right => ffi::kCGMouseButtonRight,
            Self::Center => ffi::kCGMouseButtonCenter,
            Self::Other(n) => n,
        }
    }
}

/// Where to post an event. `Hid` posts before any tap; `Session` after.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TapLocation {
    Hid,
    #[default]
    Session,
    AnnotatedSession,
}

impl TapLocation {
    pub(crate) const fn as_raw(self) -> ffi::CGEventTapLocation {
        match self {
            Self::Hid => ffi::kCGHIDEventTap,
            Self::Session => ffi::kCGSessionEventTap,
            Self::AnnotatedSession => ffi::kCGAnnotatedSessionEventTap,
        }
    }
}

/// Coordinate point on the screen, in flipped coordinates (origin
/// top-left, units = points).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<Point> for ffi::CGPoint {
    fn from(p: Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<ffi::CGPoint> for Point {
    fn from(p: ffi::CGPoint) -> Self {
        Self { x: p.x, y: p.y }
    }
}

/// A retained `CGEventRef`. Drops on scope exit.
pub struct Event {
    pub(crate) ptr: ffi::CGEventRef,
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Drop for Event {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::CFRelease(self.ptr.cast_const()) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Event {
    /// Get the event type (e.g. `kCGEventKeyDown`).
    #[must_use]
    pub fn event_type(&self) -> ffi::CGEventType {
        unsafe { ffi::CGEventGetType(self.ptr) }
    }

    /// Get the cursor position carried by this event.
    #[must_use]
    pub fn location(&self) -> Point {
        Point::from(unsafe { ffi::CGEventGetLocation(self.ptr) })
    }

    /// Get the modifier flags currently set on this event.
    #[must_use]
    pub fn flags(&self) -> ModifierFlags {
        ModifierFlags::from_bits_truncate(unsafe { ffi::CGEventGetFlags(self.ptr) })
    }

    /// Set the modifier flags on this event.
    pub fn set_flags(&self, flags: ModifierFlags) {
        unsafe { ffi::CGEventSetFlags(self.ptr, flags.bits()) };
    }

    /// Apply a unicode string to a keyboard event (so it generates that
    /// text irrespective of the keymap).
    pub fn set_unicode_string(&self, s: &str) {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        unsafe {
            ffi::CGEventKeyboardSetUnicodeString(self.ptr, utf16.len(), utf16.as_ptr());
        }
    }

    /// Read the keycode field (only meaningful for keyboard events).
    #[must_use]
    pub fn keycode(&self) -> u16 {
        let v = unsafe { ffi::CGEventGetIntegerValueField(self.ptr, ffi::kCGKeyboardEventKeycode) };
        u16::try_from(v).unwrap_or(0)
    }

    /// Read an arbitrary integer field. See `CGEventTypes.h` for valid
    /// `CGEventField` constants.
    #[must_use]
    pub fn integer_field(&self, field: ffi::CGEventField) -> i64 {
        unsafe { ffi::CGEventGetIntegerValueField(self.ptr, field) }
    }

    /// Write an arbitrary integer field.
    pub fn set_integer_field(&self, field: ffi::CGEventField, value: i64) {
        unsafe { ffi::CGEventSetIntegerValueField(self.ptr, field, value) };
    }

    /// Read this event's timestamp (Mach absolute nanoseconds since boot).
    #[must_use]
    pub fn timestamp(&self) -> u64 {
        unsafe { ffi::CGEventGetTimestamp(self.ptr) }
    }

    /// Decode the Unicode characters this event would type (mirror of
    /// `set_unicode_string`). Returns an empty string if Apple's
    /// buffer is empty.
    #[must_use]
    pub fn unicode_string(&self) -> String {
        const MAX: usize = 32;
        let mut buf = [0u16; MAX];
        let mut actual: usize = 0;
        unsafe {
            ffi::CGEventKeyboardGetUnicodeString(
                self.ptr,
                MAX,
                &mut actual,
                buf.as_mut_ptr(),
            );
        }
        String::from_utf16_lossy(&buf[..actual.min(MAX)])
    }

    /// Post the event to the requested tap location.
    pub fn post(&self, location: TapLocation) {
        unsafe { ffi::CGEventPost(location.as_raw(), self.ptr) };
    }

    /// Post the event to a specific PID.
    pub fn post_to_pid(&self, pid: i32) {
        unsafe { ffi::CGEventPostToPid(pid, self.ptr) };
    }
}

// ---- Keyboard ----

/// Build + post a keyboard event.
///
/// # Examples
///
/// ```rust,no_run
/// use cgevents::prelude::*;
/// // Press + release Cmd+A
/// KeyEvent::down(Keycode::A).with_modifiers(ModifierFlags::COMMAND).post(TapLocation::Session)?;
/// KeyEvent::up(Keycode::A).with_modifiers(ModifierFlags::COMMAND).post(TapLocation::Session)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct KeyEvent {
    keycode: u16,
    pressed: bool,
    flags: ModifierFlags,
    unicode: Option<String>,
}

impl KeyEvent {
    /// Construct a key-down event for `keycode`.
    #[must_use]
    pub const fn down(keycode: u16) -> Self {
        Self {
            keycode,
            pressed: true,
            flags: ModifierFlags::empty(),
            unicode: None,
        }
    }

    /// Construct a key-up event for `keycode`.
    #[must_use]
    pub const fn up(keycode: u16) -> Self {
        Self {
            keycode,
            pressed: false,
            flags: ModifierFlags::empty(),
            unicode: None,
        }
    }

    /// Apply modifier flags.
    #[must_use]
    pub const fn with_modifiers(mut self, flags: ModifierFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Force the event to insert this exact text — bypasses keymap
    /// translation. Useful for typing unicode that has no direct
    /// keycode (`KeyEvent::down(0).with_unicode("é")`).
    #[must_use]
    pub fn with_unicode(mut self, s: &str) -> Self {
        self.unicode = Some(s.to_string());
        self
    }

    /// Build the underlying [`Event`] without posting.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::EventCreateFailed`] if Apple returns NULL.
    pub fn build(&self, source: &EventSource) -> Result<Event, CGError> {
        let ptr = unsafe {
            ffi::CGEventCreateKeyboardEvent(source.ptr, self.keycode, self.pressed)
        };
        if ptr.is_null() {
            return Err(CGError::EventCreateFailed("CGEventCreateKeyboardEvent".into()));
        }
        let event = Event { ptr };
        if !self.flags.is_empty() {
            event.set_flags(self.flags);
        }
        if let Some(s) = &self.unicode {
            event.set_unicode_string(s);
        }
        Ok(event)
    }

    /// Build + post in one go using a fresh private [`EventSource`].
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post(&self, location: TapLocation) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post(location);
        Ok(())
    }

    /// Build + post the event to a specific process by PID. Lets you
    /// target a specific application instead of broadcasting to the
    /// whole session. (Equivalent to `CGEventPostToPid`.)
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post_to_pid(pid);
        Ok(())
    }
}

/// Build + post a mouse event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    kind: ffi::CGEventType,
    position: Point,
    button: MouseButton,
}

impl MouseEvent {
    /// Move the cursor to `position` (no button state change).
    #[must_use]
    pub const fn move_to(position: Point) -> Self {
        Self {
            kind: ffi::kCGEventMouseMoved,
            position,
            button: MouseButton::Left,
        }
    }

    /// Press `button` at `position`.
    #[must_use]
    pub const fn button_down(position: Point, button: MouseButton) -> Self {
        let kind = match button {
            MouseButton::Left => ffi::kCGEventLeftMouseDown,
            MouseButton::Right => ffi::kCGEventRightMouseDown,
            _ => ffi::kCGEventOtherMouseDown,
        };
        Self { kind, position, button }
    }

    /// Release `button` at `position`.
    #[must_use]
    pub const fn button_up(position: Point, button: MouseButton) -> Self {
        let kind = match button {
            MouseButton::Left => ffi::kCGEventLeftMouseUp,
            MouseButton::Right => ffi::kCGEventRightMouseUp,
            _ => ffi::kCGEventOtherMouseUp,
        };
        Self { kind, position, button }
    }

    /// Build the underlying [`Event`] without posting.
    ///
    /// # Errors
    ///
    /// See [`KeyEvent::build`].
    pub fn build(&self, source: &EventSource) -> Result<Event, CGError> {
        let ptr = unsafe {
            ffi::CGEventCreateMouseEvent(
                source.ptr,
                self.kind,
                self.position.into(),
                self.button.as_raw(),
            )
        };
        if ptr.is_null() {
            return Err(CGError::EventCreateFailed("CGEventCreateMouseEvent".into()));
        }
        Ok(Event { ptr })
    }

    /// Build + post in one go using a fresh private [`EventSource`].
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post(&self, location: TapLocation) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post(location);
        Ok(())
    }

    /// Build + post the mouse event to a specific process by PID.
    /// (Equivalent to `CGEventPostToPid`.)
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post_to_pid(pid);
        Ok(())
    }
}

// ---- Scroll ----

/// Build + post a scroll-wheel event.
#[derive(Debug, Clone)]
pub struct ScrollEvent {
    units: ffi::CGScrollEventUnit,
    delta_y: i32,
}

impl ScrollEvent {
    /// Scroll by `delta_y` lines (positive = up).
    #[must_use]
    pub const fn lines(delta_y: i32) -> Self {
        Self {
            units: ffi::kCGScrollEventUnitLine,
            delta_y,
        }
    }

    /// Scroll by `delta_y` pixels (positive = up).
    #[must_use]
    pub const fn pixels(delta_y: i32) -> Self {
        Self {
            units: ffi::kCGScrollEventUnitPixel,
            delta_y,
        }
    }

    /// Build the underlying [`Event`] without posting.
    ///
    /// # Errors
    ///
    /// See [`KeyEvent::build`].
    pub fn build(&self, source: &EventSource) -> Result<Event, CGError> {
        let ptr = unsafe {
            ffi::CGEventCreateScrollWheelEvent(source.ptr, self.units, 1, self.delta_y)
        };
        if ptr.is_null() {
            return Err(CGError::EventCreateFailed("CGEventCreateScrollWheelEvent".into()));
        }
        Ok(Event { ptr })
    }

    /// Build + post in one go.
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post(&self, location: TapLocation) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post(location);
        Ok(())
    }

    /// Build + post the scroll event to a specific process by PID.
    /// (Equivalent to `CGEventPostToPid`.)
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let src = EventSource::private()?;
        let ev = self.build(&src)?;
        ev.post_to_pid(pid);
        Ok(())
    }
}

// ---- Convenience: type a string ----

/// Type the string `s` as a sequence of synthesized key-down + key-up events.
///
/// Each character carries its unicode payload directly, bypassing the
/// keymap — so unicode characters with no direct keycode (e.g. `é`,
/// `中`, emoji) work too.
///
/// # Errors
///
/// See [`KeyEvent::build`].
pub fn type_string(s: &str, location: TapLocation) -> Result<(), CGError> {
    let src = EventSource::private()?;
    for ch in s.chars() {
        let chunk = ch.to_string();
        let down = KeyEvent::down(0).with_unicode(&chunk).build(&src)?;
        down.post(location);
        let up = KeyEvent::up(0).with_unicode(&chunk).build(&src)?;
        up.post(location);
    }
    Ok(())
}

// ---- Common keycodes (US QWERTY) ----

/// US-QWERTY virtual keycode constants. Use with [`KeyEvent`].
#[allow(non_snake_case, clippy::module_name_repetitions)]
pub mod Keycode {
    pub const A: u16 = 0x00;
    pub const S: u16 = 0x01;
    pub const D: u16 = 0x02;
    pub const F: u16 = 0x03;
    pub const H: u16 = 0x04;
    pub const G: u16 = 0x05;
    pub const Z: u16 = 0x06;
    pub const X: u16 = 0x07;
    pub const C: u16 = 0x08;
    pub const V: u16 = 0x09;
    pub const B: u16 = 0x0B;
    pub const Q: u16 = 0x0C;
    pub const W: u16 = 0x0D;
    pub const E: u16 = 0x0E;
    pub const R: u16 = 0x0F;
    pub const Y: u16 = 0x10;
    pub const T: u16 = 0x11;
    pub const O: u16 = 0x1F;
    pub const U: u16 = 0x20;
    pub const I: u16 = 0x22;
    pub const P: u16 = 0x23;
    pub const L: u16 = 0x25;
    pub const J: u16 = 0x26;
    pub const K: u16 = 0x28;
    pub const N: u16 = 0x2D;
    pub const M: u16 = 0x2E;

    pub const RETURN: u16 = 0x24;
    pub const TAB: u16 = 0x30;
    pub const SPACE: u16 = 0x31;
    pub const DELETE: u16 = 0x33;
    pub const ESCAPE: u16 = 0x35;
    pub const COMMAND: u16 = 0x37;
    pub const SHIFT: u16 = 0x38;
    pub const CAPS_LOCK: u16 = 0x39;
    pub const OPTION: u16 = 0x3A;
    pub const CONTROL: u16 = 0x3B;
    pub const RIGHT_SHIFT: u16 = 0x3C;
    pub const RIGHT_OPTION: u16 = 0x3D;
    pub const RIGHT_CONTROL: u16 = 0x3E;
    pub const FUNCTION: u16 = 0x3F;
    pub const F1: u16 = 0x7A;
    pub const F2: u16 = 0x78;
    pub const F3: u16 = 0x63;
    pub const F4: u16 = 0x76;
    pub const ARROW_LEFT: u16 = 0x7B;
    pub const ARROW_RIGHT: u16 = 0x7C;
    pub const ARROW_DOWN: u16 = 0x7D;
    pub const ARROW_UP: u16 = 0x7E;
}
