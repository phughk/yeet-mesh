use rand::rngs::StdRng;
use rand::SeedableRng;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// YeetMeshTest handles a single test environment, by allowing
/// test configuration and injecting the runtimes into the provided node
/// initialisation functions. The nodes then have a way to interact with
/// each other in a controlled and deterministic way.
pub struct YeetMeshTest<O> {
    prng: StdRng,
    nodes: Vec<Box<dyn Future<Output=O>>>,
}

impl<O> YeetMeshTest<O> {
    pub fn new<SEED>(seed: SEED) -> Self
    where
        SEED: Sized + Default + AsMut<[u8]>,
    {
        Self {
            prng: StdRng::from_seed(seed),
            nodes: vec![],
        }
    }

    /// Store an initialistion function that gets called when the test starts
    pub fn add_node(&mut self, node_init: Box<dyn Future<Output=O>>) {
        self.nodes.push(node_init);
    }

    /// Starts all the nodes that have been added to the cluster and runs
    /// them against the property checks
    ///
    /// This does not consume the test harness, so both the test harness
    /// and the returned future can be interacted with
    pub fn start_test(&self) -> TestProgress {
        // Start the test
    }
}

/// TestProgress is a way of controlling the execution of the cluster
struct TestProgress {}

impl<O> Future for TestProgress {
    type Output = Vec<O>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

impl TestProgress {
    /// Step the test forward by a single tick
    pub fn step(self) -> TestProgress {
        // Step the test forward
    }

    /// Step the test forward by a number of ticks
    pub fn step_by(self, ticks: usize) -> TestProgress {
        // Step the test forward by a number of ticks
    }

    /// Step the test forward until a condition is met
    pub fn step_until(self, condition: impl Fn() -> bool) -> TestProgress {
        // Step the test forward until a condition is met
    }
}
