//! File system implementation.  Six layers:
//! + Blocks: allocator for raw disk blocks. - block.rs
//! + Cache: cache for (most) in-memory blocks. - cache.rs (not yet implemented)
//! + Log: crash recovery for multi-step updates. - log.rs (not yet implemented)
//! + Inode: allocator for file system objects. - inode.rs
//! + Names: paths for convenient naming. - path.rs
//! + Files: inode allocator, reading, writing, metadata. - file.rs

use crate::sync::SpinLock;

use lazy_static::*;

/* File system interface */
pub use block::{read_as, write_as, Block};
pub use file::File;
pub use inode::{FType, Inode};
pub use path::{namei, nameiparent};

mod block;
mod cache;
mod file;
mod inode;
mod log;
mod path;

lazy_static! {
    pub static ref FS: SpinLock<FileSystem> = SpinLock::new(FileSystem::new(), "FileSystemLock");
}

pub fn init() {
    let mut fs = FS.lock();
    fs.init();
}

use block::SuperBlock;

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
