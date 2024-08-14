use std::future::Future;
use std::io::{Read, Write};
use std::net::SocketAddr;

type HostPort<'a> = (&'a str, u16);

/// Runtime is how your application interacts with the world
pub trait Runtime<CLOCK, SOCKET, LISTEN_SOCKET>
where
    CLOCK: Clock,
    SOCKET: Read + Write,
    LISTEN_SOCKET: ListenSocket<SOCKET>,
{
    /// Run async function that can move between threads
    fn run_async<O>(&self, f: Box<dyn Future<Output=O> + Send + 'static>);
    /// Run async function that is local to the current thread
    fn run_async_local<O>(&self, f: Box<dyn Future<Output=O> + 'static>);
    /// Get clock, that during tests will be controllable
    fn clock(&self) -> CLOCK;
    /// Connect to a remote node
    fn connect(&self, host_port: HostPort) -> SOCKET;
    /// Bind to a port and listen for connections
    fn bind(&self, addr: HostPort) -> LISTEN_SOCKET;
}

pub trait ListenSocket<SOCKET>
where
    SOCKET: Read + Write,
{
    fn accept(&self) -> SOCKET;
}

pub trait Clock {}
