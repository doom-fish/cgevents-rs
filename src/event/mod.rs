//! High-level `CGEvent` wrapper + builders that synthesise, inspect, and post events.

use core::ptr;

use crate::cg_event_field::CGEventField;
use crate::cg_event_flags::{CGEventFlags, ModifierFlags};
use crate::cg_event_mouse_subtype::CGEventMouseSubtype;
use crate::cg_event_tap_location::{CGEventTapLocation, TapLocation};
use crate::cg_event_timestamp::CGEventTimestamp;
use crate::cg_event_type::CGEventType;
use crate::cg_momentum_scroll_phase::CGMomentumScrollPhase;
use crate::cg_scroll_phase::CGScrollPhase;
use crate::error::CGError;
use crate::ffi;
use crate::source::EventSource;

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Center,
    Other(u32),
}

impl MouseButton {
    pub(crate) const fn as_raw(self) -> u32 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Center => 2,
            Self::Other(value) => value,
        }
    }
}

/// Coordinate point on the screen, in flipped coordinates (origin top-left, units = points).
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
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<ffi::CGPoint> for Point {
    fn from(point: ffi::CGPoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

/// A retained `CGEventRef`. Drops on scope exit.
pub struct Event {
    pub(crate) ptr: ffi::CGEventBridgeHandle,
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Drop for Event {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::cg_event::cgevent_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Event {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { ffi::cg_event::cgevent_get_type_id() }
    }

    /// Create an empty generic event.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::EventCreateFailed`] if Apple returns NULL.
    pub fn new(source: Option<&EventSource>) -> Result<Self, CGError> {
        let ptr = unsafe {
            ffi::cg_event::cgevent_create(source.map_or(ptr::null_mut(), |source| source.ptr))
        };
        if ptr.is_null() {
            Err(CGError::EventCreateFailed("CGEventCreate".into()))
        } else {
            Ok(Self { ptr })
        }
    }

    /// Serialize the event to bytes using `CGEventCreateData`.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::EventCreateFailed`] if Apple returns NULL.
    pub fn data(&self) -> Result<Vec<u8>, CGError> {
        let len = unsafe { ffi::cg_event::cgevent_create_data_length(self.ptr) };
        if len == 0 {
            return Err(CGError::BridgeError(
                "CGEventCreateData returned empty data; macOS 12+ is required for event serialization via the Swift bridge".into(),
            ));
        }
        let mut bytes = vec![0_u8; len];
        let ok =
            unsafe { ffi::cg_event::cgevent_create_data_copy(self.ptr, bytes.as_mut_ptr(), len) };
        if ok {
            Ok(bytes)
        } else {
            Err(CGError::EventCreateFailed("CGEventCreateData".into()))
        }
    }

    /// Rehydrate an event from bytes created by [`Self::data`].
    ///
    /// # Errors
    ///
    /// Returns [`CGError::EventCreateFailed`] if Apple returns NULL.
    pub fn from_data(data: &[u8]) -> Result<Self, CGError> {
        let ptr = unsafe { ffi::cg_event::cgevent_create_from_data(data.as_ptr(), data.len()) };
        if ptr.is_null() {
            Err(CGError::EventCreateFailed("CGEventCreateFromData".into()))
        } else {
            Ok(Self { ptr })
        }
    }

    /// Deep-copy the event.
    ///
    /// # Errors
    ///
    /// Returns [`CGError::EventCreateFailed`] if Apple returns NULL.
    pub fn copy(&self) -> Result<Self, CGError> {
        let ptr = unsafe { ffi::cg_event::cgevent_create_copy(self.ptr) };
        if ptr.is_null() {
            Err(CGError::EventCreateFailed("CGEventCreateCopy".into()))
        } else {
            Ok(Self { ptr })
        }
    }

    /// Build a compatible event source from this event.
    #[must_use]
    pub fn source(&self) -> Option<EventSource> {
        let ptr = unsafe { ffi::cg_event::cgevent_create_source_from_event(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(EventSource { ptr })
        }
    }

    pub fn set_source(&self, source: Option<&EventSource>) {
        unsafe {
            ffi::cg_event::cgevent_set_source(
                self.ptr,
                source.map_or(ptr::null_mut(), |source| source.ptr),
            );
        };
    }

    /// Get the raw event type (e.g. `kCGEventKeyDown`).
    #[must_use]
    pub fn event_type(&self) -> u32 {
        unsafe { ffi::cg_event::cgevent_get_type(self.ptr) }
    }

    #[must_use]
    pub fn event_type_typed(&self) -> Option<CGEventType> {
        CGEventType::from_raw(self.event_type())
    }

    pub fn set_type(&self, ty: u32) {
        unsafe { ffi::cg_event::cgevent_set_type(self.ptr, ty) };
    }

    pub fn set_event_type(&self, ty: CGEventType) {
        self.set_type(ty.raw());
    }

    /// Get the timestamp of the event.
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

    /// Get the cursor position carried by this event.
    #[must_use]
    pub fn location(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cg_event::cgevent_get_location(self.ptr, &mut x, &mut y) };
        Point::new(x, y)
    }

    /// Get the lower-left-origin coordinates carried by this event.
    #[must_use]
    pub fn unflipped_location(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cg_event::cgevent_get_unflipped_location(self.ptr, &mut x, &mut y) };
        Point::new(x, y)
    }

    pub fn set_location(&self, location: Point) {
        unsafe { ffi::cg_event::cgevent_set_location(self.ptr, location.x, location.y) };
    }

    /// Get the modifier flags currently set on this event.
    #[must_use]
    pub fn flags(&self) -> CGEventFlags {
        CGEventFlags::from_bits_truncate(unsafe { ffi::cg_event::cgevent_get_flags(self.ptr) })
    }

    pub fn set_flags(&self, flags: CGEventFlags) {
        unsafe { ffi::cg_event::cgevent_set_flags(self.ptr, flags.bits()) };
    }

    pub fn set_unicode_string(&self, s: &str) {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        unsafe {
            ffi::cg_event::cgevent_keyboard_set_unicode_string(
                self.ptr,
                utf16.as_ptr(),
                utf16.len(),
            );
        };
    }

    #[must_use]
    pub fn keycode(&self) -> u16 {
        let value = self.integer_value(CGEventField::KeyboardEventKeycode);
        u16::try_from(value).unwrap_or(0)
    }

    #[must_use]
    pub fn integer_field(&self, field: u32) -> i64 {
        unsafe { ffi::cg_event::cgevent_get_integer_value_field(self.ptr, field) }
    }

    #[must_use]
    pub fn integer_value(&self, field: CGEventField) -> i64 {
        self.integer_field(field.raw())
    }

    pub fn set_integer_field(&self, field: u32, value: i64) {
        unsafe { ffi::cg_event::cgevent_set_integer_value_field(self.ptr, field, value) };
    }

    pub fn set_integer_value(&self, field: CGEventField, value: i64) {
        self.set_integer_field(field.raw(), value);
    }

    #[must_use]
    pub fn double_field(&self, field: u32) -> f64 {
        unsafe { ffi::cg_event::cgevent_get_double_value_field(self.ptr, field) }
    }

    #[must_use]
    pub fn double_value(&self, field: CGEventField) -> f64 {
        self.double_field(field.raw())
    }

    pub fn set_double_field(&self, field: u32, value: f64) {
        unsafe { ffi::cg_event::cgevent_set_double_value_field(self.ptr, field, value) };
    }

    pub fn set_double_value(&self, field: CGEventField, value: f64) {
        self.set_double_field(field.raw(), value);
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

    pub fn post(&self, location: CGEventTapLocation) {
        unsafe { ffi::cg_event::cgevent_post(self.ptr, location.raw()) };
    }

    pub fn post_to_pid(&self, pid: i32) {
        unsafe { ffi::cg_event::cgevent_post_to_pid(self.ptr, pid) };
    }
}

/// Build + post a keyboard event.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    keycode: u16,
    pressed: bool,
    flags: ModifierFlags,
    unicode: Option<String>,
}

impl KeyEvent {
    #[must_use]
    pub const fn down(keycode: u16) -> Self {
        Self {
            keycode,
            pressed: true,
            flags: ModifierFlags::empty(),
            unicode: None,
        }
    }

    #[must_use]
    pub const fn up(keycode: u16) -> Self {
        Self {
            keycode,
            pressed: false,
            flags: ModifierFlags::empty(),
            unicode: None,
        }
    }

    #[must_use]
    pub const fn with_modifiers(mut self, flags: ModifierFlags) -> Self {
        self.flags = flags;
        self
    }

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
            ffi::cg_event::cgevent_create_keyboard_event(source.ptr, self.keycode, self.pressed)
        };
        if ptr.is_null() {
            return Err(CGError::EventCreateFailed(
                "CGEventCreateKeyboardEvent".into(),
            ));
        }
        let event = Event { ptr };
        if !self.flags.is_empty() {
            event.set_flags(self.flags);
        }
        if let Some(unicode) = &self.unicode {
            event.set_unicode_string(unicode);
        }
        Ok(event)
    }

    /// Build + post in one go using a fresh private [`EventSource`].
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post(&self, location: TapLocation) -> Result<(), CGError> {
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post(location);
        Ok(())
    }

    /// Build + post the event to a specific process by PID.
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post_to_pid(pid);
        Ok(())
    }
}

/// Build + post a mouse event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    kind: u32,
    position: Point,
    button: MouseButton,
}

impl MouseEvent {
    #[must_use]
    pub const fn move_to(position: Point) -> Self {
        Self {
            kind: CGEventType::MouseMoved.raw(),
            position,
            button: MouseButton::Left,
        }
    }

    #[must_use]
    pub const fn button_down(position: Point, button: MouseButton) -> Self {
        let kind = match button {
            MouseButton::Left => CGEventType::LeftMouseDown.raw(),
            MouseButton::Right => CGEventType::RightMouseDown.raw(),
            _ => CGEventType::OtherMouseDown.raw(),
        };
        Self {
            kind,
            position,
            button,
        }
    }

    #[must_use]
    pub const fn button_up(position: Point, button: MouseButton) -> Self {
        let kind = match button {
            MouseButton::Left => CGEventType::LeftMouseUp.raw(),
            MouseButton::Right => CGEventType::RightMouseUp.raw(),
            _ => CGEventType::OtherMouseUp.raw(),
        };
        Self {
            kind,
            position,
            button,
        }
    }

    /// Build the underlying [`Event`] without posting.
    ///
    /// # Errors
    ///
    /// See [`KeyEvent::build`].
    pub fn build(&self, source: &EventSource) -> Result<Event, CGError> {
        let ptr = unsafe {
            ffi::cg_event::cgevent_create_mouse_event(
                source.ptr,
                self.kind,
                self.position.x,
                self.position.y,
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
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post(location);
        Ok(())
    }

    /// Build + post the mouse event to a specific process by PID.
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post_to_pid(pid);
        Ok(())
    }
}

/// Build + post a scroll-wheel event.
#[derive(Debug, Clone)]
pub struct ScrollEvent {
    units: u32,
    axis1: i32,
    axis2: i32,
    axis3: i32,
}

impl ScrollEvent {
    #[must_use]
    pub const fn lines(delta_y: i32) -> Self {
        Self::lines_3d(delta_y, 0, 0)
    }

    #[must_use]
    pub const fn lines_2d(delta_y: i32, delta_x: i32) -> Self {
        Self::lines_3d(delta_y, delta_x, 0)
    }

    #[must_use]
    pub const fn lines_3d(delta_axis1: i32, delta_axis2: i32, delta_axis3: i32) -> Self {
        Self {
            units: 1,
            axis1: delta_axis1,
            axis2: delta_axis2,
            axis3: delta_axis3,
        }
    }

    #[must_use]
    pub const fn pixels(delta_y: i32) -> Self {
        Self::pixels_3d(delta_y, 0, 0)
    }

    #[must_use]
    pub const fn pixels_2d(delta_y: i32, delta_x: i32) -> Self {
        Self::pixels_3d(delta_y, delta_x, 0)
    }

    #[must_use]
    pub const fn pixels_3d(delta_axis1: i32, delta_axis2: i32, delta_axis3: i32) -> Self {
        Self {
            units: 0,
            axis1: delta_axis1,
            axis2: delta_axis2,
            axis3: delta_axis3,
        }
    }

    #[must_use]
    const fn wheel_count(&self) -> u32 {
        if self.axis3 != 0 {
            3
        } else if self.axis2 != 0 {
            2
        } else {
            1
        }
    }

    /// Build the underlying [`Event`] without posting.
    ///
    /// # Errors
    ///
    /// See [`KeyEvent::build`].
    pub fn build(&self, source: &EventSource) -> Result<Event, CGError> {
        let ptr = unsafe {
            ffi::cg_event::cgevent_create_scroll_wheel_event(
                source.ptr,
                self.units,
                self.wheel_count(),
                self.axis1,
                self.axis2,
                self.axis3,
            )
        };
        if ptr.is_null() {
            return Err(CGError::EventCreateFailed(
                "CGEventCreateScrollWheelEvent".into(),
            ));
        }
        Ok(Event { ptr })
    }

    /// Build + post in one go.
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post(&self, location: TapLocation) -> Result<(), CGError> {
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post(location);
        Ok(())
    }

    /// Build + post the scroll event to a specific process by PID.
    ///
    /// # Errors
    ///
    /// See [`Self::build`].
    pub fn post_to_pid(&self, pid: i32) -> Result<(), CGError> {
        let source = EventSource::private()?;
        let event = self.build(&source)?;
        event.post_to_pid(pid);
        Ok(())
    }
}

/// Type the string `s` as a sequence of synthesized key-down + key-up events.
///
/// # Errors
///
/// See [`KeyEvent::build`].
pub fn type_string(s: &str, location: TapLocation) -> Result<(), CGError> {
    let source = EventSource::private()?;
    for ch in s.chars() {
        let chunk = ch.to_string();
        let down = KeyEvent::down(0).with_unicode(&chunk).build(&source)?;
        down.post(location);
        let up = KeyEvent::up(0).with_unicode(&chunk).build(&source)?;
        up.post(location);
    }
    Ok(())
}

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
