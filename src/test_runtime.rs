use crate::runtime::{Clock, ConnectionSocket, HostPort, ListenSocket, Runtime};
use std::future::Future;

/// The runtime is constructed and provided to individual nodes
/// allowing for customisation of IO and execution
pub struct YeetMeshRuntime {}

impl YeetMeshRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runtime for YeetMeshRuntime {
    fn run_async<F: Future + Send>(&self, _f: F) {
        todo!()
    }

    fn run_async_local<F: Future + Send>(&self, _f: F) {
        todo!()
    }

    fn clock(&self) -> Box<&dyn Clock> {
        todo!()
    }

    fn connect(&self, _host_port: HostPort) -> Box<dyn ConnectionSocket> {
        todo!()
    }

    fn bind(&self, _addr: HostPort) -> Box<dyn ListenSocket<dyn ConnectionSocket>> {
        todo!()
    }
}
