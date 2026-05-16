#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services)
//! on macOS — synthesise, inspect, and intercept keyboard, mouse, tablet, and scroll-wheel events globally.
//!
//! The default crate surface talks to a Swift bridge that wraps CoreGraphics' C APIs.
//! Enable the `raw-ffi` feature to expose the legacy direct C imports in [`raw_ffi`].

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod cg_event_field;
pub mod cg_event_flags;
pub mod cg_event_mouse_subtype;
pub mod cg_event_tap_location;
pub mod cg_event_tap_options;
pub mod cg_event_tap_proxy;
pub mod cg_event_timestamp;
pub mod cg_event_type;
pub mod error;
pub mod event;
pub mod ffi;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod raw_ffi;
pub mod source;
pub mod tap;

pub use cg_event_field::CGEventField;
pub use cg_event_flags::{CGEventFlags, ModifierFlags, CG_EVENT_FLAGS_KNOWN_MASK};
pub use cg_event_mouse_subtype::CGEventMouseSubtype;
pub use cg_event_tap_location::{CGEventTapLocation, TapLocation};
pub use cg_event_tap_options::CGEventTapOptions;
pub use cg_event_tap_proxy::CGEventTapProxy;
pub use cg_event_timestamp::CGEventTimestamp;
pub use cg_event_type::{CGEventType, CG_ANY_INPUT_EVENT_TYPE, CG_EVENT_MASK_FOR_ALL_EVENTS};
pub use error::CGError;
pub use event::{type_string, Event, KeyEvent, Keycode, MouseButton, MouseEvent, Point, ScrollEvent};
pub use source::{EventSource, LocalEventsFilter, SourceState, SuppressionState};
pub use tap::{
    EventTap, EventTapInformation, TapAction, TapPlacement, TappedEvent,
    EVENT_TAP_ADDED_NOTIFICATION, EVENT_TAP_REMOVED_NOTIFICATION,
};

/// Common imports.
pub mod prelude {
    pub use crate::cg_event_field::CGEventField;
    pub use crate::cg_event_flags::{CGEventFlags, ModifierFlags};
    pub use crate::cg_event_mouse_subtype::CGEventMouseSubtype;
    pub use crate::cg_event_tap_location::{CGEventTapLocation, TapLocation};
    pub use crate::cg_event_tap_options::CGEventTapOptions;
    pub use crate::cg_event_timestamp::CGEventTimestamp;
    pub use crate::cg_event_type::{CGEventType, CG_EVENT_MASK_FOR_ALL_EVENTS};
    pub use crate::error::CGError;
    pub use crate::event::{type_string, Event, KeyEvent, Keycode, MouseButton, MouseEvent, Point, ScrollEvent};
    pub use crate::source::{EventSource, LocalEventsFilter, SourceState, SuppressionState};
    pub use crate::tap::{EventTap, EventTapInformation, TapAction, TapPlacement, TappedEvent};
}
