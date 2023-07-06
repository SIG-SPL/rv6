//! Inode layer of file system.

use config::fs::*;

use super::{read_block, write_block};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum FType {
    /// Directory
    Dir = 1,
    /// File
    File = 2,
    /// Device
    Device = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Inode structure
pub struct Inode {
    pub dinode: DInode,
    /// Inode number
    pub inum: u32,
    /// Device number
    pub dev: u32,
    /// Reference count
    pub refcnt: u32,
}

impl Inode {

    fn calcu_addr(inum: u32) -> (usize, usize) {
        let addr = inum as usize * core::mem::size_of::<DInode>();
        let block_num = addr / BSIZE + INDOE_START;
        let offset = addr % BSIZE;
        (block_num, offset)
    }

    pub fn get(num: u32) -> Self {
        let (block_num, offset) = Self::calcu_addr(num);
        // read the block containing the inode
        let buffer = &mut [0u8; BSIZE];
        read_block(block_num, buffer);
        // read the inode from the buffer
        let dinode = unsafe { *(buffer.as_ptr().add(offset) as *const DInode) };
        Self {
            dinode,
            inum: num,
            dev: 0,
            refcnt: 1,
        }
    }

    pub fn write(&mut self) {
        let (block_num, offset) = Self::calcu_addr(self.inum);
        // read the block containing the inode
        let buffer = &mut [0u8; BSIZE];
        read_block(block_num, buffer);
        // write the inode to the buffer
        unsafe {
            *(buffer.as_ptr().add(offset) as *mut DInode) = self.dinode;
        }
        // write the buffer back to the disk
        write_block(block_num, buffer);
    }

    /// look for a directory entry in a directory inode
    pub fn dirlookup(&mut self, _name: &str) -> Option<Inode> {
        if self.dinode.typ != FType::Dir {
            panic!("dirlookup not DIR");
        }
        todo!()
    }

    /// Write a new directory entry (name, inum) into the directory
    pub fn dirlink(&mut self, _name: &str, _inum: u32) -> Option<()> {
        if self.dinode.typ != FType::Dir {
            panic!("dirlink not DIR");
        }
        todo!()
    }
}

/// On-disk inode structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DInode {
    /// File type
    pub typ: FType,
    /// Major device number (T_DEVICE only)
    pub major: u16,
    /// Minor device number (T_DEVICE only)
    pub minor: u16,
    /// Number of links to inode in file system
    pub nlink: u16,
    /// Size of file (bytes)
    pub size: u32,
    /// Direct data block addresses
    pub addrs: [u32; config::fs::NDIRECT + 1],
}
