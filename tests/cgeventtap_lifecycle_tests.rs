use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use apple_cf::cf::CFRunLoop;
use cgevents::prelude::*;

fn passive_tap(token: &Arc<()>) -> Option<EventTap> {
    if !EventTap::preflight_listen_access() {
        eprintln!("skip: listen access not granted");
        return None;
    }
    let token = Arc::clone(token);
    match EventTap::new_with_options(
        TapLocation::Session,
        TapPlacement::TailAppend,
        CGEventTapOptions::ListenOnly,
        0,
        move |_| {
            let _ = &token;
            TapAction::Pass
        },
    ) {
        Ok(tap) => Some(tap),
        Err(CGError::TapCreateFailed) => {
            eprintln!("skip: CGEventTapCreate refused a passive tap");
            None
        }
        Err(error) => panic!("unexpected tap error: {error}"),
    }
}

#[test]
fn dropping_a_tap_releases_its_callback() {
    let token = Arc::new(());
    let Some(tap) = passive_tap(&token) else {
        return;
    };
    assert_eq!(Arc::strong_count(&token), 2);
    assert!(tap.is_enabled());
    assert!(tap.auto_reenable());
    tap.set_auto_reenable(false);
    assert!(!tap.auto_reenable());
    drop(tap);
    assert_eq!(Arc::strong_count(&token), 1);
}

#[test]
fn a_tap_dropped_from_another_thread_waits_for_its_run_loop() {
    let token = Arc::new(());
    let running = Arc::new(AtomicBool::new(true));
    let (tap_tx, tap_rx) = mpsc::channel();
    let owner = {
        let token = Arc::clone(&token);
        let running = Arc::clone(&running);
        thread::spawn(move || {
            let tap = passive_tap(&token);
            drop(token);
            let created = tap.is_some();
            tap_tx.send(tap).expect("send tap");
            while created && running.load(Ordering::SeqCst) {
                let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(20), false);
            }
        })
    };

    let tap = tap_rx.recv().expect("tap from owner thread");
    if let Some(tap) = tap {
        thread::sleep(Duration::from_millis(50));
        let started = Instant::now();
        drop(tap);
        assert!(started.elapsed() < Duration::from_secs(1));
        assert_eq!(Arc::strong_count(&token), 1);
    }
    running.store(false, Ordering::SeqCst);
    owner.join().expect("owner thread");
}

#[test]
fn a_tap_dropped_after_its_thread_exited() {
    let token = Arc::new(());
    let tap = {
        let token = Arc::clone(&token);
        thread::spawn(move || passive_tap(&token))
            .join()
            .expect("owner thread")
    };
    let Some(tap) = tap else {
        return;
    };
    let started = Instant::now();
    drop(tap);
    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(Arc::strong_count(&token), 1);
}
