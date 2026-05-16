# cgevents

Safe Rust bindings for Apple's [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services) on macOS — synthesise, inspect, and intercept keyboard, mouse, tablet, and scroll-wheel events globally.

> **Status:** v0.5 ships a Swift-first bridge for `CGEvent`, `CGEventSource`, `CGEventTap`, `CGEventField`, `CGEventType`, `CGEventFlags`, `CGEventMouseSubtype`, `CGEventTapLocation`, `CGEventTapOptions`, `CGEventTapProxy`, and `CGEventTimestamp`. The legacy direct C surface remains available behind the `raw-ffi` feature.

## Highlights

- Swift bridge by default; raw C imports moved behind `raw-ffi`.
- Typed Rust wrappers for `CGEventType`, `CGEventField`, `CGEventFlags`, `CGEventMouseSubtype`, `CGEventTapLocation`, `CGEventTapOptions`, `CGEventTapProxy`, and `CGEventTimestamp`.
- Safe wrappers for event creation, copying, serialisation, source extraction, tap creation, tap inventory, and Accessibility preflight/request helpers.
- 11 runnable examples + 11 per-area smoke tests.

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
    event.set_integer_value(CGEventField::MouseEventDeltaX, 12);
    event.set_mouse_subtype(CGEventMouseSubtype::TabletPoint);

    println!("state_id={} flags={:?}", source.source_state_id(), source.flags_state());
    println!("event_type={:?} location={:?}", event.event_type_typed(), event.location());
    println!("mouse_subtype={:?}", event.mouse_subtype());
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

## `raw-ffi` feature

Enable `raw-ffi` when you need the direct CoreGraphics C imports instead of the Swift-first safe surface:

```toml
[dependencies]
cgevents = { version = "0.5", features = ["raw-ffi"] }
```

This exposes `cgevents::raw_ffi` with the legacy `extern "C"` declarations, constants, and structs.

## Permissions

- `CGEventPost` synthesis does not require Accessibility permission.
- `CGEventTap` interception usually requires Accessibility permission.
- `EventTap::{preflight,request}_listen_access` and `EventTap::{preflight,request}_post_access` wrap the 10.15+ access helpers.

## Notes

- `Event::data()` / `Event::from_data()` use the Swift overlay's event-data bridge. On macOS 12+ this is fully supported by the default Swift bridge. The direct C entry points remain available behind `raw-ffi` for lower-level callers.
- Deprecated PSN tap/post APIs remain intentionally omitted from the safe surface; see `COVERAGE.md`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
