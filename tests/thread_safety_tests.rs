use cgevents::async_api::CGEventTapStream;
use cgevents::{Event, EventSource, EventTap};

trait AmbiguousIfSync<A> {
    fn marker() {}
}

impl<T: ?Sized> AmbiguousIfSync<()> for T {}

struct IsSync;

impl<T: ?Sized + Sync> AmbiguousIfSync<IsSync> for T {}

const fn assert_send<T: Send>() {}

const fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn events_and_sources_are_send_but_not_sync() {
    assert_send::<Event>();
    assert_send::<EventSource>();
    let _ = <Event as AmbiguousIfSync<_>>::marker;
    let _ = <EventSource as AmbiguousIfSync<_>>::marker;
}

#[test]
fn taps_and_streams_are_send_and_sync() {
    assert_send_sync::<EventTap>();
    assert_send_sync::<CGEventTapStream>();
}
