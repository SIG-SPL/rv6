//! Inode layer of file system.

use super::{
    block::{BitMap, Dir},
    read_as, write_as, Block,
};
use config::fs::*;

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
    pub fn root() -> Self {
        Self::get(ROOTINO).unwrap()
    }

    /// Allocate a new inode with the given type and major/minor number.
    pub fn new(typ: FType, major: u16, minor: u16) -> Self {
        let dinode = DInode {
            typ,
            major,
            minor,
            nlink: 1,
            size: 0,
            addrs: [0; config::fs::NDIRECT + 1],
        };
        // Allocate a new inode in bitmap
        let mut bitmap = Block::read_block(INODE_BITMAP_START);
        let inum = bitmap.alloc().expect("Inode::new: no inodes");
        // Write the new inode to disk
        let inode = Self {
            dinode,
            inum,
            dev: 0,
            refcnt: 1,
        };
        inode.write_back();
        inode
    }

    /// Copy a modified in-memory inode to disk.
    pub fn free(&mut self) {
        if self.refcnt == 0 {
            panic!("freeing free inode");
        }
        self.refcnt -= 1;
        if self.refcnt == 0 {
            let mut bitmap = Block::read_block(INODE_BITMAP_START);
            bitmap.set(self.inum, 0);
        }
    }

    fn calcu_addr(inum: u32) -> (usize, usize) {
        let addr = inum as usize * core::mem::size_of::<DInode>();
        let block_num = addr / BSIZE + INDOE_START;
        let offset = addr % BSIZE;
        (block_num, offset)
    }

    /// Get an inode from disk
    pub fn get(num: u32) -> Option<Self> {
        let (block_num, offset) = Self::calcu_addr(num);
        // read the block containing the inode
        let block = Block::read_block(block_num);
        // read the inode from the buffer
        let dinode = read_as(&block, offset);
        Some(Self {
            dinode,
            inum: num,
            dev: 0,
            refcnt: 1,
        })
    }

    /// Write an inode to disk
    pub fn write_back(&self) {
        let (block_num, offset) = Self::calcu_addr(self.inum);
        // read the block containing the inode
        let mut block = Block::new(block_num);
        // write the inode to the buffer
        write_as(&mut block, offset, self.dinode);
    }

    /// look for a directory entry in a directory inode
    pub fn dirlookup(&self, name: &str) -> Option<Inode> {
        if self.dinode.typ != FType::Dir {
            panic!("dirlookup not DIR");
        }
        let block = Block::read_block(self.dinode.addrs[0] as usize);
        block.dirlookup(name, self.dinode.size as usize)
    }

    /// Write a new directory entry (name, inum) into the directory
    pub fn dirlink(&mut self, name: &str, inum: u32) -> Option<()> {
        if self.dinode.typ != FType::Dir {
            panic!("dirlink not DIR");
        }
        let mut block = Block::read_block(self.dinode.addrs[0] as usize);
        block.dirlink(name, inum, self.dinode.size as usize)
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
