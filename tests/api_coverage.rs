//! API-surface coverage harness for `cgevents`.
//!
//! `Quartz Event Services` lives inside `CoreGraphics` as a pure-C header
//! triplet (`CGEvent.h`, `CGEventSource.h`, `CGEventTypes.h`). Mirrors
//! the apple-cf / videotoolbox / imageio C-function-regex pattern.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_header(name: &str) -> String {
    let p = sdk_root().join(format!(
        "System/Library/Frameworks/CoreGraphics.framework/Headers/{name}.h"
    ));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

fn extract_c_functions(prefix: &str, source: &str) -> BTreeSet<String> {
    let pattern = format!(r"\b({prefix}[A-Za-z0-9_]+)\s*\(");
    let re = regex_lite::Regex::new(&pattern).unwrap();
    re.captures_iter(source).map(|c| c[1].to_string()).collect()
}

fn extract_rust_externs() -> BTreeSet<String> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    let s = std::fs::read_to_string(&p).unwrap();
    let re = regex_lite::Regex::new(r"pub\s+fn\s+([A-Za-z0-9_]+)\s*\(").unwrap();
    re.captures_iter(&s).map(|c| c[1].to_string()).collect()
}

fn report(
    name: &str,
    apple: &BTreeSet<String>,
    ours: &BTreeSet<String>,
    omitted: &BTreeSet<String>,
) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|s| !omitted.contains(*s))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

#[test]
fn cg_event_coverage() {
    let header = read_header("CGEvent");
    let apple = extract_c_functions("CGEvent", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        // Type-id boilerplate.
        "CGEventGetTypeID",
        // Less-common reads we don't surface in v0.1 (callers can read
        // raw u64 timestamps via CGEventGetTimestamp instead):
        "CGEventCreateData",
        "CGEventCreateFromData",
        "CGEventCreateCopy",
        "CGEventCreateSourceFromEvent",
        "CGEventGetUnflippedLocation",
        "CGEventGetDoubleValueField",
        "CGEventSetDoubleValueField",
        "CGEventSetSource",
        "CGEventSetType",
        "CGEventSetTimestamp",
        "CGEventPostToPSN",
        // Tap variants we don't (yet) wrap — v0.2 will surface
        // CGEventTapCreateForPid + CGEventTapCreateForPSN + the AnnotatedSession variant.
        "CGEventTapCreateForPid",
        "CGEventTapCreateForPSN",
        "CGEventTapPostEvent",
        // Multi-axis scroll variant — v0.2. Single-axis scroll covered by
        // CGEventCreateScrollWheelEvent.
        "CGEventCreateScrollWheelEvent2",
    ]);
    report("CGEvent", &apple, &ours, &omitted);
}

#[test]
fn cg_event_source_coverage() {
    let header = read_header("CGEventSource");
    let apple = extract_c_functions("CGEventSource", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        "CGEventSourceGetTypeID",
        // Tunables (key-repeat threshold, pixel pressure, etc.) — v0.2.
        "CGEventSourceGetSourceStateID",
        "CGEventSourceKeyState",
        "CGEventSourceFlagsState",
        "CGEventSourceCounterForEventType",
        "CGEventSourceSecondsSinceLastEventType",
        "CGEventSourceButtonState",
        "CGEventSourceSetLocalEventsFilterDuringSuppressionState",
        "CGEventSourceGetLocalEventsFilterDuringSuppressionState",
        "CGEventSourceGetLocalEventsSuppressionInterval",
        "CGEventSourceSetLocalEventsSuppressionInterval",
        "CGEventSourceGetPixelsPerLine",
        "CGEventSourceSetPixelsPerLine",
        "CGEventSourceGetUserData",
        "CGEventSourceSetUserData",
        "CGEventSourceSetKeyboardType",
        "CGEventSourceGetKeyboardType",
        "CGEventSourceCreateInputSource",
    ]);
    report("CGEventSource", &apple, &ours, &omitted);
}
