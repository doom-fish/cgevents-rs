#![cfg(feature = "raw-ffi")]

use cgevents::raw_ffi;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).expect("utf8 stdout").trim().to_string())
}

fn read_header(name: &str) -> String {
    let path = sdk_root().join(format!(
        "System/Library/Frameworks/ApplicationServices.framework/Frameworks/CoreGraphics.framework/Headers/{name}.h"
    ));
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn extract_c_functions(prefix: &str, source: &str) -> BTreeSet<String> {
    let pattern = format!(r"\b({prefix}[A-Za-z0-9_]+)\s*\(");
    let regex = regex_lite::Regex::new(&pattern).expect("regex");
    regex.captures_iter(source).map(|capture| capture[1].to_string()).collect()
}

fn extract_rust_externs() -> BTreeSet<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/raw_ffi.rs");
    let source = std::fs::read_to_string(&path).expect("read raw_ffi.rs");
    let regex = regex_lite::Regex::new(r"pub\s+fn\s+([A-Za-z0-9_]+)\s*\(").expect("regex");
    regex.captures_iter(&source).map(|capture| capture[1].to_string()).collect()
}

fn assert_coverage(name: &str, apple: &BTreeSet<String>, ours: &BTreeSet<String>) {
    let missing: BTreeSet<_> = apple.difference(ours).cloned().collect();
    assert!(missing.is_empty(), "{name} missing symbols: {missing:?}");
}

#[test]
fn cg_event_raw_ffi_coverage() {
    assert_eq!(raw_ffi::cg_event_mask_bit(raw_ffi::kCGEventKeyDown), 1_u64 << raw_ffi::kCGEventKeyDown);
    assert_coverage(
        "CGEvent",
        &extract_c_functions("CGEvent", &read_header("CGEvent")),
        &extract_rust_externs(),
    );
}

#[test]
fn cg_event_source_raw_ffi_coverage() {
    assert_coverage(
        "CGEventSource",
        &extract_c_functions("CGEventSource", &read_header("CGEventSource")),
        &extract_rust_externs(),
    );
}
