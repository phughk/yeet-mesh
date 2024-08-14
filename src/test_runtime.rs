use crate::runtime::Runtime;
use crate::test_network::YeetMeshSocket;
use std::future::Future;

/// The runtime is constructed and provided to individual nodes
/// allowing for customisation of IO and execution
pub(super) struct YeetMeshRuntime {}

impl YeetMeshRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl<CLOCK, LISTEN_SOCKET> Runtime<CLOCK, YeetMeshSocket, LISTEN_SOCKET> for YeetMeshRuntime {
    fn run_async<O>(&self, f: Box<dyn Future<Output = O> + Send + 'static>) {
        todo!()
    }

    fn run_async_local<O>(&self, f: Box<dyn Future<Output = O> + 'static>) {
        todo!()
    }

    fn clock(&self) -> CLOCK {
        todo!()
    }

    fn connect(&self, host_port: crate::runtime::HostPort) -> YeetMeshSocket {
        todo!()
    }

    fn bind(&self, addr: crate::runtime::HostPort) -> LISTEN_SOCKET {
        todo!()
    }
}
