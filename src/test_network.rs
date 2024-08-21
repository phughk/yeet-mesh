use crate::runtime::ListenSocket;
use std::io::{Read, Write};

/// A network interface that tracks peer-to-peer connections
pub(super) struct YeetMeshNetwork {}

trait NetworkInterface {}

impl YeetMeshNetwork {
    pub fn new() -> Self {
        YeetMeshNetwork {}
    }

    pub fn new_interface(&self, _hostname: String) {}
}

/// YeetMeshSocket represents a peer to peer connection, whether initated from a client
/// or accepted from a bind+listen
pub(super) struct YeetMeshSocket {}

impl Read for YeetMeshSocket {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl Write for YeetMeshSocket {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

pub(super) struct YeetMeshListenSocket {}

impl ListenSocket<YeetMeshSocket> for YeetMeshListenSocket {
    fn accept(&self) -> YeetMeshSocket {
        todo!()
    }
}
