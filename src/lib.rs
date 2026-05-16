#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services)
//! on macOS — synthesise + inspect keyboard, mouse, and scroll-wheel
//! events globally.
//!
//! Pure C — zero Swift bridge.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod event;
pub mod ffi;
pub mod source;
pub mod tap;

pub use error::CGError;
pub use event::{
    type_string, Event, KeyEvent, Keycode, ModifierFlags, MouseButton, MouseEvent, Point,
    ScrollEvent, TapLocation,
};
pub use source::{EventSource, LocalEventsFilter, SourceState, SuppressionState};
pub use tap::{EventTap, TapAction, TappedEvent};

/// Common imports.
pub mod prelude {
    pub use crate::error::CGError;
    pub use crate::event::{
        type_string, Event, KeyEvent, Keycode, ModifierFlags, MouseButton, MouseEvent, Point,
        ScrollEvent, TapLocation,
    };
    pub use crate::source::{EventSource, LocalEventsFilter, SourceState, SuppressionState};
    pub use crate::tap::{EventTap, TapAction, TappedEvent};
}
