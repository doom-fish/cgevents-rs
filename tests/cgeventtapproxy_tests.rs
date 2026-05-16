use cgevents::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[test]
#[ignore = "requires Accessibility permission and live tap delivery"]
fn cgeventtapproxy_can_repost_event() -> Result<(), Box<dyn std::error::Error>> {
    if !EventTap::preflight_listen_access() {
        return Ok(());
    }

    let seen = Arc::new(AtomicBool::new(false));
    let seen_in_callback = Arc::clone(&seen);
    let tap = EventTap::new_with_options(
        TapLocation::Session,
        TapPlacement::HeadInsert,
        CGEventTapOptions::Default,
        CGEventType::KeyDown.mask_bit(),
        move |event| {
            if event.keycode() == Keycode::A && !seen_in_callback.swap(true, Ordering::SeqCst) {
                if let Ok(source) = EventSource::private() {
                    if let Ok(key_up) = KeyEvent::up(Keycode::A).build(&source) {
                        event.proxy().post_event(&key_up);
                    }
                }
                EventTap::stop_current_run_loop();
            }
            TapAction::Pass
        },
    )?;

    thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_millis(50));
            let _ = KeyEvent::down(Keycode::A).post(TapLocation::Session);
        });
        scope.spawn(|| {
            thread::sleep(Duration::from_millis(250));
            tap.stop();
        });
        tap.run();
    });

    assert!(seen.load(Ordering::SeqCst));
    Ok(())
}
