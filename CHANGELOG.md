# Changelog

## [0.5.1] - 2026-05-16

### Added

- `CGGesturePhase`, `CGMomentumScrollPhase`, and `CGScrollPhase` safe enums plus `Event` / `TappedEvent` helpers for scroll-phase and momentum-phase fields.
- Raw `raw-ffi` constants for every `CGGesturePhase`, `CGMomentumScrollPhase`, and `CGScrollPhase` symbol in `CGEventTypes.h`.
- Swift bridge validation helpers and a smoke test covering the phase enums against the SDK overlay.

### Changed

- `COVERAGE.md` and `COVERAGE_AUDIT.md` now report full public macOS `CGEventTypes.h` coverage aside from the deprecated `CGEventPostToPSN` exemption.

## [0.5.0] - 2026-05-16

### Added

- Swift-first `CGEventsBridge` package plus Cargo build integration modelled on the ScreenCaptureKit bridge pattern.
- Safe Rust coverage for `CGEvent`, `CGEventSource`, `CGEventTap`, `CGEventField`, `CGEventType`, `CGEventFlags`, `CGEventMouseSubtype`, `CGEventTapLocation`, `CGEventTapOptions`, `CGEventTapProxy`, and `CGEventTimestamp`.
- Per-area examples (`01`-`11`) and per-area smoke tests, including a permission-gated tap-proxy repost flow.
- `EventTap::installed`, `EventTap::for_pid`, tap access preflight/request helpers, typed tap inventory, and `TapPlacement`.
- `raw-ffi` feature exposing the legacy direct C imports separately from the default Swift bridge.
- `COVERAGE.md` audit of `CGEvent.h`, `CGEventSource.h`, and `CGEventTypes.h` event-system coverage.

### Changed

- Default bindings now flow through Swift holder objects instead of linking raw C directly from the safe surface.
- README now documents the typed enum/flag areas, the `raw-ffi` feature, and the tap access helpers.
- Internal linking now pulls the Swift compatibility/runtime libraries required by non-Swift Rust binaries.

## [0.4.0] - 2026-05-16

### Added

- Full raw FFI coverage for the current public `CGEvent` and `CGEventSource` C functions.
- `Event` helpers for serialization (`data`, `from_data`), copying, source extraction, unflipped coordinates, double-valued fields, and source/type/timestamp mutation.
- `EventSource` helpers for keyboard type, pixels-per-line, user data, source-state queries, counters, time-since-last-event, and local-event suppression tuning.
- `LocalEventsFilter` bitflags and `SuppressionState` enum.
- Multi-axis `ScrollEvent::{lines_2d, lines_3d, pixels_2d, pixels_3d}` support via `CGEventCreateScrollWheelEvent2`.
- `TappedEvent::post` for reposting synthetic events from an active tap callback.
- Smoke examples for event round-tripping and event-source state inspection.

### Changed

- README status/roadmap now reflects the shipped event/source inspection surface.
- Package contents now include examples and tests.
- Coverage tests now target the full current `CGEvent` + `CGEventSource` headers without omissions.

## [0.1.0] - Initial release

### Added

- `KeyEvent::{down, up}` builders with `with_modifiers`, `with_unicode`.
- `MouseEvent::{move_to, button_down, button_up}` builders.
- `ScrollEvent::{lines, pixels}` builders.
- `type_string(s, location)` helper for Unicode + emoji input that bypasses the keymap.
- `EventTap::{new, keyboard, mouse}` for event interception with `TapAction::{Pass, Drop}`.
- `EventSource` with `Private`/`CombinedSession`/`HIDSystem` states.
- `Event` accessor type with `event_type`, `location`, `flags`, `keycode`.
- `Point`, `MouseButton`, `TapLocation`, `ModifierFlags` (bitflags).
- `Keycode` module with ~25 common US-QWERTY constants.
- `CGError` variants: `EventCreateFailed`, `TapCreateFailed`, `SourceCreateFailed`, `InvalidArgument`.
- 3 examples (`01_synth_click`, `02_type_string`, `03_keyboard_tap`).
- 2 API-coverage tests (`CGEvent`, `CGEventSource`) using the apple-cf / videotoolbox / imageio C-function-regex harness.
