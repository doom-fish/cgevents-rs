# cgevents

Safe Rust bindings for Apple's [Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz_event_services) on macOS — synthesise, inspect, and intercept keyboard, mouse, tablet, and scroll-wheel events globally.

> **Status:** v0.11 makes event taps safe to drop from any thread or from inside their own callback, re-enables taps that the system disables, lets a tap callback replace an event, and returns `CGError::PostAccessDenied` instead of posting an event the system would drop. The Swift-first bridge covers `CGEvent`, `CGEventSource`, `CGEventTap`, `CGEventField`, `CGEventType`, `CGEventFlags`, `CGEventMouseSubtype`, `CGGesturePhase`, `CGMomentumScrollPhase`, `CGScrollPhase`, `CGEventTapLocation`, `CGEventTapOptions`, `CGEventTapProxy`, and `CGEventTimestamp`; the `async` feature adds `CGEventTapStream`, and the legacy direct C surface remains available behind the `raw-ffi` feature.

Requires macOS 10.15 or later.

## Highlights

- Swift bridge by default; raw C imports moved behind `raw-ffi`.
- Typed Rust wrappers for `CGEventType`, `CGEventField`, `CGEventFlags`, `CGEventMouseSubtype`, `CGGesturePhase`, `CGMomentumScrollPhase`, `CGScrollPhase`, `CGEventTapLocation`, `CGEventTapOptions`, `CGEventTapProxy`, and `CGEventTimestamp`.
- Safe wrappers for event creation, copying, serialisation, source extraction, typed scroll/momentum phase inspection, tap creation, tap inventory, and Accessibility preflight/request helpers.
- **`async` feature** — `CGEventTapStream` wraps `CGEventTapCreate` as a `BoundedAsyncStream<CGEventItem>` with a dedicated run-loop thread and RAII unsubscribe.
- 12 runnable examples and per-area test suites.

## Quick start — async event stream

```rust,no_run
use cgevents::async_api::CGEventTapStream;
use cgevents::{TapLocation, CG_EVENT_MASK_FOR_ALL_EVENTS};

# async fn run() -> Result<(), cgevents::CGError> {
let stream = CGEventTapStream::subscribe(
    TapLocation::Session,
    CG_EVENT_MASK_FOR_ALL_EVENTS,
    64,
)?;
while let Some(ev) = stream.next().await {
    println!("{:?}  @ ({:.0}, {:.0})", ev.event_type, ev.location.x, ev.location.y);
}
# Ok(())
# }
```

Add to `Cargo.toml`:

```toml
cgevents = { version = "0.11", features = ["async"] }
```

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
    tap.run()?;
    Ok(())
}
```

## `raw-ffi` feature

Enable `raw-ffi` when you need the direct CoreGraphics C imports instead of the Swift-first safe surface:

```toml
[dependencies]
cgevents = { version = "0.11", features = ["raw-ffi"] }
```

This exposes `cgevents::raw_ffi` with the legacy `extern "C"` declarations, constants, and structs.

## Event taps

- A tap is serviced by the run loop of the thread that created it. `EventTap::run` runs that loop and returns `CGError::WrongThread` on any other thread; `EventTap::stop` works from any thread, even before `run` starts.
- `EventTap` is `Send` and `Sync`. Dropping it from another thread removes the tap and waits (up to two seconds) for its run loop to finish a callback in progress; dropping it inside its own callback is also safe.
- When the system disables a tap (`kCGEventTapDisabledByTimeout` or `ByUserInput`), the callback sees that event type and the tap is re-enabled automatically; `EventTap::set_auto_reenable(false)` turns that off.
- A callback returns `TapAction::Pass`, `TapAction::Drop`, or `TapAction::Replace(event)` to substitute another event.

## Permissions

- Posting events (`Event::post`, `Event::post_to_pid`, `CGEventTapProxy::post_event`, `TappedEvent::post`, the builders' `post` helpers and `type_string`) requires the Accessibility permission on macOS 10.15 and later. Each call checks it with `CGPreflightPostEventAccess` first and, when it is missing, returns `CGError::PostAccessDenied` without posting, instead of handing the system an event it would drop silently. `EventTap::request_post_access()` asks for the permission. The check is a privacy-database query that takes a millisecond or two, so `type_string` makes it once for the whole string.
- Intercepting events with a filtering tap requires the Accessibility permission; a listen-only tap that observes keyboard events requires Input Monitoring (`EventTap::preflight_listen_access`, `EventTap::request_listen_access`). Tap creation returns `CGError::TapCreateFailed` without them.

## Notes

- A keyboard event carries at most 20 UTF-16 code units of text (`MAX_UNICODE_STRING_LENGTH`); `set_unicode_string` and `KeyEvent::build` reject longer strings with `CGError::InvalidArgument`. Use `type_string` for longer text.
- `Event` and `EventSource` are `Send` but not `Sync`: their setters take `&self`, so share one across threads only behind a lock.
- `Event::data()` / `Event::from_data()` use the Swift overlay's event-data bridge. On macOS 12+ this is fully supported by the default Swift bridge. The direct C entry points remain available behind `raw-ffi` for lower-level callers.
- Deprecated PSN tap/post APIs remain intentionally omitted from the safe surface; see `COVERAGE.md`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
