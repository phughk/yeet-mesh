use crate::test_harness::YeetMeshTest;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// TestProgress is a way of controlling the execution of the cluster
///
/// This is effectively a single-threaded async Executor with fine-grained controls
pub struct TestProgress<'a, STATE, OUTPUT, F> {
    pub(crate) runtime: &'a YeetMeshTest<STATE, OUTPUT, F>,
    pub(crate) futures: Vec<Box<dyn Future<Output = Box<dyn Any>>>>,
    pub(crate) states: Vec<Box<dyn Fn() -> STATE>>,
}

impl<'a, STATE, OUTPUT, F> Future for TestProgress<'a, STATE, OUTPUT, F> {
    type Output = Vec<OUTPUT>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

impl<'a, STATE, OUTPUT> TestProgress<'a, STATE, OUTPUT> {
    /// Step the test forward by a single async poll
    pub fn step(&mut self) {
        // Step the test forward
    }

    /// Step the test forward by a number of async polls
    pub fn step_by(&mut self, ticks: usize) {
        // Step the test forward by a number of ticks
        for _ in 0..ticks {
            self.step();
        }
    }

    /// Step the test forward until a condition is met
    pub fn step_until(&mut self, _condition: impl Fn() -> bool) {
        // Step the test forward until a condition is met
        todo!("Unimplemented step until")
    }
}
