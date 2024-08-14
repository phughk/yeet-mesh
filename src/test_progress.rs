use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// TestProgress is a way of controlling the execution of the cluster
///
/// This is effectively a single-threaded async Executor with fine-grained controls
pub(super) struct TestProgress {}

impl<O> Future for TestProgress {
    type Output = Vec<O>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

impl TestProgress {
    /// Step the test forward by a single async poll
    pub fn step(self) -> TestProgress {
        // Step the test forward
    }

    /// Step the test forward by a number of async polls
    pub fn step_by(self, ticks: usize) -> TestProgress {
        // Step the test forward by a number of ticks
    }

    /// Step the test forward until a condition is met
    pub fn step_until(self, condition: impl Fn() -> bool) -> TestProgress {
        // Step the test forward until a condition is met
    }
}
