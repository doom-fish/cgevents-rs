# Changelog

## [0.1.0] - Initial release

### Added

- `KeyEvent::{down, up}` builders with `with_modifiers`, `with_unicode`.
- `MouseEvent::{move_to, button_down, button_up}` builders.
- `ScrollEvent::{lines, pixels}` builders.
- `type_string(s, location)` helper for Unicode + emoji input that
  bypasses the keymap.
- `EventTap::{new, keyboard, mouse}` for event interception with
  `TapAction::{Pass, Drop}`.
- `EventSource` with `Private`/`CombinedSession`/`HIDSystem` states.
- `Event` accessor type with `event_type`, `location`, `flags`, `keycode`.
- `Point`, `MouseButton`, `TapLocation`, `ModifierFlags` (bitflags).
- `Keycode` module with ~25 common US-QWERTY constants.
- `CGError` variants: `EventCreateFailed`, `TapCreateFailed`,
  `SourceCreateFailed`, `InvalidArgument`.
- 3 examples (`01_synth_click`, `02_type_string`, `03_keyboard_tap`).
- 2 API-coverage tests (`CGEvent`, `CGEventSource`) using the apple-cf /
  videotoolbox / imageio C-function-regex harness.

### Why zero-Swift?

Quartz Event Services is pure C — `extern "C"` declarations link against
CoreGraphics directly. No Swift bridge, no static-lib build step, no
runtime rpath setup.
