use crate::runtime::Clock;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub(super) struct YeetMeshClock {}

impl Clock for YeetMeshClock {
    fn sleep(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'static>> {
        Box::pin(YeetMeshSleepFuture {})
    }
}

pub struct YeetMeshSleepFuture {}

impl Future for YeetMeshSleepFuture {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
