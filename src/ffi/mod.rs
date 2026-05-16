//! Swift-bridge FFI declarations for Quartz Event Services.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::c_void;

pub type CGEventBridgeHandle = *mut c_void;
pub type CGEventSourceBridgeHandle = *mut c_void;
pub type CGEventTapBridgeHandle = *mut c_void;
pub type CGEventTapProxyBridgeHandle = *mut c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CGEventTapInformation {
    pub event_tap_id: u32,
    pub tap_point: u32,
    pub options: u32,
    pub events_of_interest: u64,
    pub tapping_process: i32,
    pub process_being_tapped: i32,
    pub enabled: bool,
    pub min_usec_latency: f32,
    pub avg_usec_latency: f32,
    pub max_usec_latency: f32,
}

pub mod cg_event;
pub mod cg_event_field;
pub mod cg_event_flags;
pub mod cg_event_mouse_subtype;
pub mod cg_event_source;
pub mod cg_event_tap;
pub mod cg_event_tap_location;
pub mod cg_event_tap_options;
pub mod cg_event_tap_proxy;
pub mod cg_event_timestamp;
pub mod cg_event_type;
