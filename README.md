# cgevents

Safe Rust bindings for Apple's [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services) on macOS — synthesise + intercept keyboard, mouse, and scroll-wheel events globally.

> **Status:** actively developed. v0.4 ships keyboard / mouse / scroll synthesis, unicode `type_string`, event tap interception, event/source inspection helpers, multi-axis scroll creation, and common US-QWERTY keycodes.

Pure C — **zero Swift bridge** (like `videotoolbox`, `imageio`).

## Quick start — synthesise input

```rust,no_run
use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    KeyEvent::down(Keycode::A)
        .with_modifiers(ModifierFlags::COMMAND)
        .post(TapLocation::Session)?;
    KeyEvent::up(Keycode::A)
        .with_modifiers(ModifierFlags::COMMAND)
        .post(TapLocation::Session)?;

    type_string("Hello, 🌍 世界\n", TapLocation::Session)?;
    MouseEvent::move_to(Point::new(500.0, 300.0)).post(TapLocation::Session)?;
    ScrollEvent::pixels_2d(80, 20).post(TapLocation::Session)?;
    Ok(())
}
```

## Quick start — inspect events and sources

```rust,no_run
use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    source.set_user_data(42);

    let event = MouseEvent::move_to(Point::new(400.0, 300.0)).build(&source)?;
    let bytes = event.data()?;
    let copy = Event::from_data(&bytes)?.copy()?;

    println!("state_id={} flags={:?}", source.source_state_id(), source.flags_state());
    println!("event at {:?} / {:?}", copy.location(), copy.unflipped_location());
    Ok(())
}
```

## Quick start — intercept input

```rust,no_run
use cgevents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tap = EventTap::keyboard(|event| {
        println!("keycode={} flags={:?}", event.keycode(), event.flags());
        TapAction::Pass
    })?;
    tap.run();
    Ok(())
}
```

## Permissions

* **`CGEventPost`** — needs **no permission**.
* **`CGEventTap`** (interception) — requires **Accessibility permission**.

## Roadmap

- [x] `KeyEvent::{down, up}` with modifier flags
- [x] `MouseEvent::{move_to, button_down, button_up}`
- [x] `ScrollEvent::{lines, pixels}` plus multi-axis `*_2d` / `*_3d`
- [x] `type_string` for unicode-payload input
- [x] `EventTap::{new, keyboard, mouse}` with `Pass`/`Drop` actions
- [x] `Event` helpers for copy, serialization, source extraction, and location/field access
- [x] `EventSource` helpers for keyboard type, pixels-per-line, user data, source-state queries, and suppression tuning
- [x] `CGEventPostToPid` ergonomics on `Event`, `KeyEvent`, `MouseEvent`, and `ScrollEvent`
- [ ] Per-process event taps (`CGEventTapCreateForPid`) convenience wrappers
- [ ] Tablet-pointer + tablet-proximity event builders
- [ ] Full Apple keycode set (beyond the common US-QWERTY subset)
- [ ] Permission-helper that opens System Settings to the Accessibility pane

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
