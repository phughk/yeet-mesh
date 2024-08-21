use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;

pub(crate) type HostPort<'a> = (&'a str, u16);

pub trait ConnectionSocket: Read + Write {}

/// Runtime is how your application interacts with the world
pub trait Runtime {
    /// Run async function that can move between threads
    fn run_async<F: Future + Send>(&self, f: F);
    /// Run async function that is local to the current thread
    fn run_async_local<F: Future + Send>(&self, f: F);
    /// Get clock, that during tests will be controllable
    fn clock(&self) -> Box<&dyn Clock>;
    /// Connect to a remote node
    fn connect(&self, host_port: HostPort) -> Box<dyn ConnectionSocket>;
    /// Bind to a port and listen for connections
    fn bind(&self, addr: HostPort) -> Box<dyn ListenSocket<dyn ConnectionSocket>>;
}

pub trait ListenSocket<SOCKET>
where
    SOCKET: Read + Write,
{
    fn accept(&self) -> SOCKET;
}

pub trait Clock {
    fn sleep(
        &self,
        duration: std::time::Duration,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;
}
