use cgevents::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

fn main() {
    run();
}

fn run() {
    if !EventTap::preflight_listen_access() {
        println!("listen access unavailable; skipping tap-proxy example");
        return;
    }

    let seen = Arc::new(AtomicBool::new(false));
    let seen_in_callback = Arc::clone(&seen);
    let tap = match EventTap::new_with_options(
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
    ) {
        Ok(tap) => tap,
        Err(error) => {
            println!("tap creation failed ({error}); skipping tap-proxy example");
            return;
        }
    };

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
    println!("proxy_reposted=true");
}
