# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.11.0] - Unreleased

### Security

- Event taps no longer free their callback state under a running callback. 0.10.1 freed the context as soon as the `EventTap` dropped, and the unreleased follow-up fix only retained the tap's Swift holder inside the callback, so a drop on another thread between the dispatch and that retain was still a use-after-free. The tap's `userInfo` now holds a retained `doom_fish_utils::callback_context::CallbackContext` for as long as the tap can call back, the callback takes no retain, and teardown finalises the context on the tap's own run loop (waiting for a callback in progress when the drop happens on another thread, and deferring the release when it happens inside the callback).

### Fixed

- A tap disabled by `kCGEventTapDisabledByTimeout` or `kCGEventTapDisabledByUserInput` stayed disabled for good. Both `EventTap` and `CGEventTapStream` now re-enable it with `CGEventTapEnable`, and `TappedEvent::event_type` reports the type the callback was invoked with, so callbacks see these notifications instead of the event's own `CGEventGetType` value.
- `EventTap::stop` issued before `EventTap::run` started was discarded by Core Foundation and `run` then blocked forever; the stop request now takes effect when the loop starts. `CGEventTapStream` uses the same stop request.
- `CGEventTapStream::subscribe` with a capacity of zero returns `CGError::InvalidArgument` instead of panicking.
- `raw_ffi::CGEventCreateScrollWheelEvent` is declared variadic, like the C function; calls with more than one wheel passed the extra deltas incorrectly on arm64.
- The README said posting events needs no permission. It needs the Accessibility permission since macOS 10.15, and events are dropped silently without it; the README now says so and points at `EventTap::preflight_post_access`.
- The stream bridge no longer force-unwraps `CFRunLoopGetCurrent`.

### Changed

- **BREAKING:** `EventTap::run` returns `Result<(), CGError>`. It runs the tap's own run loop and returns the new `CGError::WrongThread` on any thread other than the one that created the tap; it used to run the calling thread's loop.
- **BREAKING:** `Event` and `EventSource` are no longer `Sync`, because their setters take `&self` and Core Graphics doesn't synchronise them. Both remain `Send`.
- **BREAKING:** `Event::set_unicode_string` and `TappedEvent::set_unicode_string` return `Result<(), CGError>` and reject strings longer than 20 UTF-16 code units, which delivered keyboard events cannot carry; `KeyEvent::build` returns that error for `with_unicode` strings over the limit.
- **BREAKING:** `TapAction` gained a variant that carries an `Event`, so it only derives `Debug` now (no `Clone`, `PartialEq` or `Eq`).
- **BREAKING:** the Swift-bridge declarations in `cgevents::ffi::cg_event_tap` changed: `RustTapCallback` takes a replacement-event out-parameter, the context hooks are `unsafe extern "C" fn`, and `cgevent_tap_run(tap)` replaces `cgevent_tap_run_current_run_loop`.
- **BREAKING:** requires `apple-cf` 0.11 (`>=0.11, <0.12`) and `doom-fish-utils` 0.4.1 (`>=0.4.1, <0.5`); `rust-version` is 1.82.
- Dropping an `EventTap` on a thread other than its own waits up to two seconds for a callback in progress on the tap's run loop.
- A tap callback that panicked is still called for later events; its lock used to stay poisoned, so every later event passed through without reaching it.

### Added

- `TapAction::Replace(Event)`, which hands the event system a new event in place of the intercepted one.
- `EventTap::set_auto_reenable` and `EventTap::auto_reenable` (on by default).
- `MAX_UNICODE_STRING_LENGTH`, and a `Debug` implementation for `Event`.

## [0.10.1] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.10.0] - 2026-05-18

### Changed

- Re-exported `CFIndex`, `CFTypeID`, `CFTimeInterval`, `CGCharCode`, and `CGKeyCode` from `apple_cf::raw`, removing five duplicate local CF/CG typedefs from `raw_ffi`.
- Widened `apple-cf` to `>=0.9, <0.10` and bumped the crate to `0.10.0`.

## [0.9.0] - 2026-05-18

### Changed

- Re-exported `CFAllocatorRef`, `CFDataRef`, `CFMachPortRef`, `CFRunLoopRef`, `CFRunLoopSourceRef`, `CFStringRef`, and `CFTypeRef` from `apple_cf::raw` in both `raw_ffi` and `ffi`, removing seven duplicate local Core Foundation reference typedefs.
- Bumped the crate to `0.9.0` for the shared `apple-cf` raw Core Foundation surface.

## [0.8.0] - 2026-05-18

### Changed

- Bumped the crate to `0.8.0` to track `apple-cf` 0.8.x while keeping the shared `apple_cf::cg::CGPoint` surface unchanged.
- Audited internal `CGRect` usage for the nested-layout update; no `CGRect` call sites or legacy field accessors required changes.

## [0.7.0] - 2026-05-18

### Changed

- Re-export `apple_cf::cg::CGPoint` from both `src/raw_ffi.rs` and `src/ffi/mod.rs`, removing the duplicate local definitions and aligning the crate's Core Graphics point type with the shared `apple-cf` surface.
- Added `apple-cf = { version = ">=0.7, <0.9", default-features = false, features = ["cg"] }`.

## [0.6.1] - 2026-05-17

### Fixed

- **Thread-exit race in `CGEventTapStream` drop** — `cgevents_tap_stream_unsubscribe`
  now calls `stopAndJoin()` before releasing the Swift bridge object, guaranteeing
  that the dedicated run-loop thread has fully exited before the Rust `Drop` impl
  frees the `AsyncStreamSender` pointer.  Previously `CFRunLoopStop` was called
  without waiting, leaving a window where an in-flight callback could observe a
  freed pointer.
- **Panic guard on `EventTap` trampoline** — the `extern "C"` trampoline that
  invokes user `FnMut` callbacks now wraps the call in
  `doom_fish_utils::panic_safe::catch_user_panic`, preventing a user panic from
  unwinding across the FFI boundary (UB).
- **SAFETY comment** added to the raw-pointer dereference inside `trampoline`.
- **`doom-fish-utils` version range** tightened to `>=0.1, <0.3`.

## [0.6.0] - 2026-05-17

### Added

- **`async` feature** — `src/async_api.rs` module gated behind `features = ["async"]`.
- **`CGEventTapStream`** — executor-agnostic, lossy-by-default async stream wrapping `CGEventTapCreate`. Installs a listen-only tap on a dedicated run-loop thread; the Rust `Drop` impl disables the tap, stops the thread, and closes the stream.
- **`CGEventItem`** — owned event snapshot (`event_type`, `location`, `flags`, `timestamp`, `keycode`) suitable for crossing thread boundaries.
- Swift bridge addition: `AsyncStream.swift` with `CGEventsTapAsyncStreamBridge`, `cgevents_tap_stream_subscribe`, and `cgevents_tap_stream_unsubscribe`.
- New dependency: `doom-fish-utils = { path = "../doom-fish-utils", version = "0.1" }`.
- New dev-dependency: `pollster = "0.3"` (for examples).
- Example `12_async_tap_stream` — async keyboard tap with 2-second graceful timeout.
- Test suite `async_stream_tests` — 6 tests covering send/sync, subscribe/drop, try_next, capacity, and permission-failure paths.

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
