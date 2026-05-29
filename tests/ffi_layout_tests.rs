//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift bridge.
//!
//! `CGEventTapInformation` is marshalled by value through the Swift `@_cdecl`
//! FFI boundary (filled in by `FFIEventTapInformation` on the Swift side and
//! read back by `EventTap::installed` on the Rust side). If its size or
//! alignment ever drifts from what Swift expects, the data marshalling silently
//! corrupts. These tests pin the layout so accidental field reordering / type
//! changes are caught at `cargo test` time rather than as runtime garbage.

use std::mem::{align_of, size_of};

use cgevents::ffi::{cgevent_verify_ffi_layout, CGEventTapInformation};

#[test]
fn cgevent_tap_information_layout() {
    // u32, u32, u32, (pad 4), u64, i32, i32, bool, (pad 3), f32, f32, f32, (pad 4)
    assert_eq!(
        size_of::<CGEventTapInformation>(),
        48,
        "CGEventTapInformation size drifted"
    );
    assert_eq!(
        align_of::<CGEventTapInformation>(),
        8,
        "CGEventTapInformation alignment drifted"
    );
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its*
/// `MemoryLayout` (size/stride/alignment) for `FFIEventTapInformation` matches
/// the values pinned on the Rust side. A `false` return means the Rust and
/// Swift layouts genuinely disagree, which is a real ABI bug.
#[test]
fn ffi_layout_matches_swift() {
    // SAFETY: `cgevent_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { cgevent_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
