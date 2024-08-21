use crate::runtime::Runtime;
use crate::test_progress::TestProgress;
use crate::test_runtime::YeetMeshRuntime;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::future::Future;
use std::mem;
use std::sync::{Arc, Mutex};

/// NodeInit is effectively an initializer that accepts a runtime and returns a
/// synchronous state polling function alongside the future
type NodeInit<R, STATE, OUTPUT, F>
where
    R: Runtime,
    OUTPUT: Send,
    F: Future<Output = OUTPUT> + Send,
= Box<dyn FnOnce(R) -> (NodeInitStateSupplier<STATE>, NodeInitEntryPoint<OUTPUT, F>)>;

/// NodeInitStateSupplier is a function that returns the current state of a node
type NodeInitStateSupplier<STATE> = Box<dyn Fn() -> STATE>;

/// NodeInitEntryPoint is a function that is called with the runtime and is the future of the
/// entrypoint of the node
type NodeInitEntryPoint<OUTPUT, F: Future<Output = OUTPUT> + Send> = Box<F>;

/// YeetMeshTest handles a single test environment, by allowing
/// test configuration and injecting the runtimes into the provided node
/// initialisation functions. The nodes then have a way to interact with
/// each other in a controlled and deterministic way.
pub struct YeetMeshTest<STATE, OUTPUT, F> {
    prng: Mutex<StdRng>,
    // Node inits are nodes yet to be initialised
    // Nodes can be added during execution
    node_init: Mutex<Vec<NodeInit<YeetMeshRuntime, STATE, OUTPUT, F>>>,
}

impl<STATE, OUTPUT: Send + 'static, F> YeetMeshTest<STATE, OUTPUT, F> {
    pub fn new<SEED>(mut seed: SEED) -> Self
    where
        SEED: Sized + Default + AsMut<[u8]>,
    {
        let mut real_seed = [0u8; 32];
        real_seed.as_mut().copy_from_slice(seed.as_mut());
        let rng = StdRng::from_seed(real_seed);
        Self {
            prng: Mutex::new(rng),
            node_init: Mutex::new(vec![]),
        }
    }

    /// Store an initialization function that gets called when the test starts
    ///
    /// When additional nodes are added after test start, they will be started in
    /// random deterministic iterations
    pub fn add_node(&mut self, node_init: NodeInit<YeetMeshRuntime, STATE, OUTPUT, F>) {
        self.node_init
            .try_lock()
            .map_err(|e| {
                format!(
                    "Failed to add node because the test nodes list is locked: {}",
                    e
                )
            })
            .unwrap()
            .push(node_init);
    }

    /// Starts all the nodes that have been added to the cluster and runs
    /// them against the property checks.
    /// This is effectively running all futures with a poll
    ///
    /// This does not consume the test harness, so both the test harness
    /// and the returned future can be interacted with
    pub fn start_test(&self) -> TestProgress<STATE, OUTPUT> {
        // Start the test
        let mut nodes = self.node_init.try_lock().unwrap();
        let futures = Arc::new(Mutex::new(Vec::with_capacity(nodes.len())));
        let mut states = vec![];
        // We replace the vec behind the mutex with an empty one
        let nodes = { mem::take(&mut *nodes) };
        for node in nodes.into_iter() {
            let runtime = YeetMeshRuntime {};
            let (state, future) = node(runtime);
            let futures = futures.clone();
            let (_runnable, _task) =
                async_task::spawn(future, move |res| futures.try_lock().unwrap().push(res));
            states.push(state);
        }
        TestProgress {
            runtime: self,
            futures: vec![],
            states,
        }
    }
}
