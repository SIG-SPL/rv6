//! File system implementation.  Five layers:
//! + Blocks: allocator for raw disk blocks.
//! + Log: crash recovery for multi-step updates. (not yet implemented)
//! + Files: inode allocator, reading, writing, metadata.
//! + Directories: inode with special contents (list of other inodes!)
//! + Names: paths for convenient naming.

use crate::sync::SpinLock;
use config::fs::*;
use lazy_static::*;
use virtio_drivers::device::blk::SECTOR_SIZE;

/* File system interface */
pub use file::File;
pub use inode::{FType, Inode};
pub use path::{namei, nameiparent};

mod file;
mod inode;
mod path;

lazy_static! {
    pub static ref FS: SpinLock<FileSystem> = SpinLock::new(FileSystem::new(), "FileSystemLock");
}

pub fn init() {
    let mut fs = FS.lock();
    fs.init();
}

pub struct FileSystem {
    sb: SuperBlock,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            sb: SuperBlock::new(),
        }
    }

    fn init(&mut self) {
        self.sb = SuperBlock::init();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SuperBlock {
    /// Must be FSMAGIC
    magic: u32,
    /// Size of file system image (blocks)
    size: u32,
    /// Number of data blocks
    nblocks: u32,
    /// Number of inodes.
    ninodes: u32,
    /// Number of log blocks
    nlog: u32,
    /// Number of blocks in inode file
    logstart: u32,
    /// Block number of first inode block
    inodestart: u32,
    /// Block number of first free map block
    bmapstart: u32,
}

impl SuperBlock {
    fn new() -> Self {
        Self {
            magic: 0,
            size: 0,
            nblocks: 0,
            ninodes: 0,
            nlog: 0,
            logstart: 0,
            inodestart: 0,
            bmapstart: 0,
        }
    }

    fn init() -> Self {
        let buf = &mut [0u8; BSIZE];
        read_block(1, buf);
        let sb = unsafe { *(buf.as_ptr() as *const SuperBlock) };
        println!("sb: {:?}", sb);
        sb
    }
}

/// Disk layer read_block
fn read_block(blockno: usize, buf: &mut [u8]) {
    use crate::io::virtio::block;
    assert_eq!(buf.len(), BSIZE);
    let sector_num = BSIZE / SECTOR_SIZE;
    for i in 0..sector_num {
        block::read(
            blockno * sector_num + i,
            &mut buf[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
        )
        .unwrap()
    }
}

/// Disk layer write_block
fn write_block(blockno: usize, buf: &[u8]) {
    use crate::io::virtio::block;
    assert_eq!(buf.len(), BSIZE);
    let sector_num = BSIZE / SECTOR_SIZE;
    for i in 0..sector_num {
        block::write(
            blockno * sector_num + i,
            &buf[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
        )
        .unwrap();
    }
}
