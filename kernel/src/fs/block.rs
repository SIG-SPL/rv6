use config::fs::*;
use virtio_drivers::device::blk::SECTOR_SIZE;

use super::Inode;

pub struct Block {
    pub(super) data: [u8; BSIZE],
    pub(super) blockno: usize,
}

impl Drop for Block {
    fn drop(&mut self) {
        self.write_back();
    }
}

impl Block {
    pub fn new(blockno: usize) -> Self {
        Self {
            data: [0u8; BSIZE],
            blockno,
        }
    }

    /// Disk layer read_block
    pub fn read_block(blockno: usize) -> Self {
        let mut buf = [0u8; BSIZE];
        use crate::io::virtio::block;
        let sector_num = BSIZE / SECTOR_SIZE;
        for i in 0..sector_num {
            block::read(
                blockno * sector_num + i,
                &mut buf[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
            )
            .unwrap()
        }
        Self { data: buf, blockno }
    }

    /// Disk layer write_block
    /// never use this function directly, use drop instead
    fn write_back(&self) {
        use crate::io::virtio::block;
        let sector_num = BSIZE / SECTOR_SIZE;
        for i in 0..sector_num {
            block::write(
                self.blockno * sector_num + i,
                &self.data[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
            )
            .unwrap();
        }
    }
}

pub fn read_as<T>(block: &Block, offset: usize) -> T
where
    T: Copy,
{
    unsafe { *(block.data.as_ptr().add(offset) as *const T) }
}

pub fn write_as<T>(block: &mut Block, offset: usize, value: T) {
    unsafe {
        *(block.data.as_ptr().add(offset) as *mut T) = value;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuperBlock {
    /// Must be FSMAGIC
    pub magic: u32,
    /// Size of file system image (blocks)
    pub size: u32,
    /// Number of data blocks
    pub nblocks: u32,
    /// Number of inodes.
    pub ninodes: u32,
    /// Number of log blocks
    pub nlog: u32,
    /// Number of blocks in inode file
    pub logstart: u32,
    /// Block number of first inode block
    pub inodestart: u32,
    /// Block number of first free map block
    pub bmapstart: u32,
}

impl SuperBlock {
    pub fn new() -> Self {
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

    pub fn init() -> Self {
        let block = Block::read_block(SUPER_BLOCK_NO);
        let sb = unsafe { *(block.data.as_ptr() as *const SuperBlock) };
        assert_eq!(sb.magic, FS_MAGIC);
        sb
    }
}

impl Default for SuperBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub trait BitMap {
    /// Set the index-th bit as value.
    fn set(&mut self, index: u32, value: u8);
    /// Allocate a free bit, return None if no free bit.
    fn alloc(&mut self) -> Option<u32>;
    /// Get the index-th bit.
    fn get(&self, index: u32) -> u8;
}

const FREE: u8 = 0;

impl BitMap for Block {
    fn set(&mut self, index: u32, value: u8) {
        let block = (index / 8) as usize;
        let offset = index % 8;
        let mask = 1 << offset;
        if value == 0 {
            self.data[block] &= !mask;
        } else {
            self.data[block] |= mask;
        }
    }

    fn alloc(&mut self) -> Option<u32> {
        for (i, &b) in self.data.iter().enumerate() {
            for j in 0..8 {
                let mask = 1 << j;
                if b & mask == FREE {
                    self.data[i] |= mask;
                    return Some((i * 8 + j) as u32);
                }
            }
        }
        None
    }

    fn get(&self, index: u32) -> u8 {
        let block = index / 8;
        let offset = index % 8;
        let mask = 1 << offset;
        (self.data[block as usize] & mask) >> offset
    }
}

pub trait Dir {
    /// Return the size of the directory used space in bytes.
    fn size(&self) -> u32;
    /// Look for a directory entry named name in the directory.
    fn dirlookup(&self, name: &str) -> Option<Inode>;
    /// Write a new directory entry (name, inum) into the directory.
    fn dirlink(&mut self, name: &str, inum: u32) -> Option<()>;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirEntry {
    pub inum: u32,
    pub name: [u8; DIRSIZ],
}

impl DirEntry {
    fn new(inum: u32, name: &str) -> Self {
        let mut name_bytes = [0u8; DIRSIZ];
        name_bytes[..name.len()].copy_from_slice(name.as_bytes());
        Self {
            inum,
            name: name_bytes,
        }
    }
}

impl Dir for Block {
    // use first 4 bytes to store size
    fn size(&self) -> u32 {
        read_as::<u32>(self, 0)
    }

    fn dirlookup(&self, path: &str) -> Option<Inode> {
        let size = self.size() as usize;
        let mut offset = 4;
        while offset < size {
            let entry = unsafe { *(self.data.as_ptr().add(offset) as *const DirEntry) };
            if entry.inum == 0 {
                return None;
            }
            let name = unsafe { core::str::from_utf8_unchecked(&entry.name) };
            if path == name {
                return Inode::get(entry.inum);
            }
            offset += core::mem::size_of::<DirEntry>();
        }
        None
    }

    fn dirlink(&mut self, name: &str, inum: u32) -> Option<()> {
        if name.len() > DIRSIZ {
            return None;
        }
        let size = self.size() as usize;
        let mut offset = 4;
        while offset < size {
            let entry: DirEntry = read_as(self, offset);
            let entry_name = unsafe {
                core::str::from_utf8_unchecked(&entry.name)
                    .strip_suffix('\0')
                    .unwrap_or_default()
            };
            if entry_name == name {
                return None;
            }
            offset += core::mem::size_of::<DirEntry>();
        }
        let dir = DirEntry::new(inum, name);
        write_as(self, size, dir);
        let new_size = size + core::mem::size_of::<DirEntry>();
        write_as(self, 0, new_size as u32);
        Some(())
    }
}
