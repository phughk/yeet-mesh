use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;

pub(crate) type HostPort<'a> = (&'a str, u16);

pub trait ConnectionSocket: Read + Write {}

/// Runtime is how your application interacts with the world
pub trait Runtime {
    /// Run async function that can move between threads
    fn run_async<O>(&self, f: Box<dyn Future<Output = O> + Send + 'static>);
    /// Run async function that is local to the current thread
    fn run_async_local<O>(&self, f: Box<dyn Future<Output = O> + 'static>);
    /// Get clock, that during tests will be controllable
    fn clock(&self) -> &dyn Clock;
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
