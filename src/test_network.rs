use std::io::{Read, Write};

/// A network interface that tracks peer-to-peer connections
pub(super) struct YeetMeshNetwork {}

trait NetworkInterface {}

impl YeetMeshNetwork {
    pub fn new() -> Self {
        YeetMeshNetwork {}
    }

    pub fn new_interface(&self, hostname: String) {}
}

pub(super) struct YeetMeshSocket {}

impl Read for YeetMeshSocket {}

impl Write for YeetMeshSocket {}
