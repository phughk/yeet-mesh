use crate::runtime::{Clock, Runtime};
use crate::test_harness::YeetMeshTest;
use crate::test_runtime::YeetMeshRuntime;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct TestApp {}

#[derive(Default, Clone, Copy)]
pub struct TestAppState {
    pub counter: u32,
}

#[test]
fn test_simple() {
    let mut test = YeetMeshTest::new([0u8; 32]);
    test.add_node(Box::new(create_app));
}

fn create_app(
    runtime: YeetMeshRuntime,
) -> (
    Box<dyn Fn() -> TestAppState>,
    Pin<Box<dyn Future<Output = ()> + Send>>,
) {
    let internal_state = Arc::new(Mutex::new(TestAppState::default()));
    let state = internal_state.clone();
    let state = Box::new(move || {
        let state = state.lock().unwrap().clone();
        state
    });
    let future = Box::pin(async move {
        for i in 0..10 {
            runtime.clock().sleep(Duration::from_secs(1)).await.unwrap();
            let mut internal_state = internal_state.try_lock().unwrap();
            internal_state.counter += 1;
        }
    });
    (state, future)
}
