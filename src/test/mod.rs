use crate::runtime::{Clock, Runtime};
use crate::test_harness::YeetMeshTest;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

pub struct TestApp<R>
where
    R: Runtime,
{
    state: Arc<Mutex<TestAppState>>,
    runtime: RwLock<Option<R>>,
}

impl<R: Runtime> TestApp<R> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(TestAppState::default())),
            runtime: RwLock::new(None),
        }
    }

    pub async fn run(&self, runtime: R) {
        self.runtime.try_write().unwrap().replace(runtime);
        let _read_lock = self.runtime.try_read().unwrap();
        let runtime = _read_lock.as_ref().unwrap();
        for _ in 0..10 {
            runtime.clock().sleep(Duration::from_secs(1)).await.unwrap();
            let mut internal_state = self.state.try_lock().unwrap();
            internal_state.counter += 1;
        }
    }

    /// a factory method for the function. This needs to be written this way to avoid a &self reference
    fn get_state_function(&self) -> Box<dyn Fn() -> TestAppState> {
        let state = self.state.clone();
        Box::new(move || state.try_lock().unwrap().clone())
    }
}

#[derive(Default, Clone, Copy)]
pub struct TestAppState {
    pub counter: u32,
}

#[test]
fn test_simple() {
    // Create "cluster"
    let mut test = YeetMeshTest::new([0u8; 32]);
    // Create node instance
    let app = TestApp::new();
    // Add node to cluster, indicating how to get its status and entrypoint
    test.add_node(Box::new(|r| {
        (app.get_state_function(), Box::new(app.run(r)))
    }));
    // Start the cluster test
    let mut test_progress = test.start_test();
    // Do some execution
    test_progress.step_by(10);
}
