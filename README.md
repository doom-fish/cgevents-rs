# cgevents

Safe Rust bindings for Apple's [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services) on macOS — synthesise + intercept keyboard, mouse, and scroll-wheel events globally.

> **Status:** experimental. v0.1 ships keyboard / mouse / scroll synthesis, unicode `type_string`, event tap interception, and ~25 common US-QWERTY keycodes. Per-pixel mouse delta + tablet-pointer events + clipboard integration land in v0.2.

Pure C — **zero Swift bridge** (like `videotoolbox`, `imageio`).

## Quick start — synthesise input

```rust,no_run
use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Press Cmd+A
    KeyEvent::down(Keycode::A)
        .with_modifiers(ModifierFlags::COMMAND)
        .post(TapLocation::Session)?;
    KeyEvent::up(Keycode::A)
        .with_modifiers(ModifierFlags::COMMAND)
        .post(TapLocation::Session)?;

    // Type a string (works with unicode + emoji — bypasses keymap)
    type_string("Hello, 🌍 世界\n", TapLocation::Session)?;

    // Mouse: move + click
    MouseEvent::move_to(Point::new(500.0, 300.0)).post(TapLocation::Session)?;
    MouseEvent::button_down(Point::new(500.0, 300.0), MouseButton::Left)
        .post(TapLocation::Session)?;
    MouseEvent::button_up(Point::new(500.0, 300.0), MouseButton::Left)
        .post(TapLocation::Session)?;

    // Scroll up 5 lines
    ScrollEvent::lines(5).post(TapLocation::Session)?;
    Ok(())
}
```

## Quick start — intercept input

```rust,no_run
use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tap = EventTap::keyboard(|event| {
        println!("keycode={} flags={:?}", event.keycode(), event.flags());
        TapAction::Pass  // or TapAction::Drop to swallow it
    })?;
    tap.run();  // blocks the current thread's run loop
    Ok(())
}
```

## Permissions

* **`CGEventPost`** — needs **no permission**. Synthesised events go through cleanly.
* **`CGEventTap`** (interception) — requires **Accessibility permission** (System Settings → Privacy & Security → Accessibility). The first run will surface `CGError::TapCreateFailed`; grant the permission, restart, and the tap will work.

## Pipeline composition

```text
cgevents (synth) ──► another macOS app
cgevents (tap)   ◄── physical keyboard/mouse
                       │
                       ▼
                custom hotkey handler
                custom macro recorder
                custom remap engine (Karabiner-style)
```

Pairs naturally with [`screencapturekit`](https://github.com/doom-fish/screencapturekit-rs) (record what the synthesized input does) and [`carbonhotkey`](https://github.com/doom-fish/carbonhotkey-rs) (lighter-weight hotkey-only interception).

## Roadmap

- [x] `KeyEvent::{down, up}` with modifier flags
- [x] `MouseEvent::{move_to, button_down, button_up}`
- [x] `ScrollEvent::{lines, pixels}`
- [x] `type_string` for unicode-payload input
- [x] `EventTap::{new, keyboard, mouse}` with `Pass`/`Drop` actions
- [x] ~25 common US-QWERTY keycode constants
- [ ] All ~120 documented Apple keycodes (full enum)
- [ ] `CGEventPostToPid` ergonomics + per-app event injection helper
- [ ] Tablet-pointer + tablet-proximity events
- [ ] Async `EventTap` driver that doesn't require its own run loop
- [ ] Permission-helper that opens System Settings to the Accessibility pane

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
