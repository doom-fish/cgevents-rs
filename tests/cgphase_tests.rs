use cgevents::prelude::*;

#[test]
fn cgphase_bridge_values_and_scroll_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let source = EventSource::private()?;
    let event = ScrollEvent::pixels_2d(8, -3).build(&source)?;
    event.set_scroll_phase(CGScrollPhase::Began);
    event.set_momentum_scroll_phase(CGMomentumScrollPhase::Begin);

    assert_eq!(event.scroll_phase(), Some(CGScrollPhase::Began));
    assert_eq!(
        event.momentum_scroll_phase(),
        Some(CGMomentumScrollPhase::Begin)
    );
    assert_eq!(CGScrollPhase::from_raw(0), None);
    assert_eq!(CGMomentumScrollPhase::from_raw(u32::MAX), None);
    assert_eq!(CGGesturePhase::from_raw(3), None);

    let scroll_expected = [
        CGScrollPhase::Began,
        CGScrollPhase::Changed,
        CGScrollPhase::Ended,
        CGScrollPhase::Cancelled,
        CGScrollPhase::MayBegin,
    ];
    for (index, phase) in scroll_expected.into_iter().enumerate() {
        let index = u32::try_from(index).expect("scroll phase index fits in u32");
        assert_eq!(
            unsafe { cgevents::ffi::cg_scroll_phase::cgscroll_phase_raw_value(index) },
            phase.raw()
        );
        assert_eq!(CGScrollPhase::from_raw(phase.raw()), Some(phase));
    }

    let momentum_expected = [
        CGMomentumScrollPhase::None,
        CGMomentumScrollPhase::Begin,
        CGMomentumScrollPhase::Continue,
        CGMomentumScrollPhase::End,
    ];
    for (index, phase) in momentum_expected.into_iter().enumerate() {
        let index = u32::try_from(index).expect("momentum phase index fits in u32");
        assert_eq!(
            unsafe {
                cgevents::ffi::cg_momentum_scroll_phase::cgmomentum_scroll_phase_raw_value(index)
            },
            phase.raw()
        );
        assert_eq!(CGMomentumScrollPhase::from_raw(phase.raw()), Some(phase));
    }

    let gesture_expected = [
        CGGesturePhase::None,
        CGGesturePhase::Began,
        CGGesturePhase::Changed,
        CGGesturePhase::Ended,
        CGGesturePhase::Cancelled,
        CGGesturePhase::MayBegin,
    ];
    for (index, phase) in gesture_expected.into_iter().enumerate() {
        let index = u32::try_from(index).expect("gesture phase index fits in u32");
        assert_eq!(
            unsafe { cgevents::ffi::cg_gesture_phase::cggesture_phase_raw_value(index) },
            phase.raw()
        );
        assert_eq!(CGGesturePhase::from_raw(phase.raw()), Some(phase));
    }

    Ok(())
}
