use crate::runtime::{Clock, ConnectionSocket, HostPort, ListenSocket, Runtime};
use std::future::Future;

/// The runtime is constructed and provided to individual nodes
/// allowing for customisation of IO and execution
pub(super) struct YeetMeshRuntime {}

impl YeetMeshRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runtime for YeetMeshRuntime {
    fn run_async<O>(&self, f: Box<dyn Future<Output = O> + Send + 'static>) {
        todo!()
    }

    fn run_async_local<O>(&self, f: Box<dyn Future<Output = O> + 'static>) {
        todo!()
    }

    fn clock(&self) -> &dyn Clock {
        todo!()
    }

    fn connect(&self, host_port: HostPort) -> Box<dyn ConnectionSocket> {
        todo!()
    }

    fn bind(&self, addr: HostPort) -> Box<dyn ListenSocket<dyn ConnectionSocket>> {
        todo!()
    }
}
