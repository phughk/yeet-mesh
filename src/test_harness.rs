use crate::runtime::Runtime;
use crate::test_network::YeetMeshSocket;
use crate::test_progress::TestProgress;
use crate::test_runtime::YeetMeshRuntime;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::future::Future;
use std::sync::Mutex;

/// YeetMeshTest handles a single test environment, by allowing
/// test configuration and injecting the runtimes into the provided node
/// initialisation functions. The nodes then have a way to interact with
/// each other in a controlled and deterministic way.
pub struct YeetMeshTest<O> {
    prng: Mutex<StdRng>,
    nodes: Vec<Box<dyn Future<Output = O>>>,
}

/// NodeInit is effectively an initializer that accepts a runtime and returns a
/// synchronous state polling function alongside the future
type NodeInit<R, CLOCK, SOCKET, LIST_SOCK>
where
    R: Runtime<CLOCK, SOCKET, LIST_SOCK>,
= Box<dyn FnOnce(R) -> ()>;

impl<O> YeetMeshTest<O> {
    pub fn new<SEED>(seed: SEED) -> Self
    where
        SEED: Sized + Default + AsMut<[u8]>,
    {
        Self {
            prng: Mutex::new(StdRng::from_seed(seed)),
            nodes: vec![],
        }
    }

    /// Store an initialization function that gets called when the test starts
    ///
    /// TODO: It would be really cool, if a node were able to provide a status struct
    /// Such a struct can be used to verify invariants
    pub fn add_node(
        &mut self,
        node_init: NodeInit<YeetMeshRuntime, CLOCK, YeetMeshSocket, LIST_SOCK>,
    ) {
        self.nodes.push(node_init);
    }

    /// Starts all the nodes that have been added to the cluster and runs
    /// them against the property checks.
    /// This is effectively running all futures with a poll
    ///
    /// This does not consume the test harness, so both the test harness
    /// and the returned future can be interacted with
    pub fn start_test(&self) -> TestProgress {
        // Start the test
    }
}
