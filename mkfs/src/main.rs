//! mkfs: create a file system image
//! Usage: mkfs fs.img files ...
//! The file system image is fs.img.  The files are copied into the
//! file system image.
//!
//! The code is adapted from the xv6 file system implementation.

use std::{
    env,
    fs::File,
    io::{Seek, Write},
    process::exit,
};

use config::fs::*;

pub const IPB: u32 = BSIZE as u32 / std::mem::size_of::<DInode>() as u32;
pub const NINODEBLOCKS: u32 = NINODES / IPB + 1;

/// On-disk inode structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DInode {
    /// File type
    pub typ: u16,
    /// Major device number (T_DEVICE only)
    pub major: u16,
    /// Minor device number (T_DEVICE only)
    pub minor: u16,
    /// Number of links to inode in file system
    pub nlink: u16,
    /// Size of file (bytes)
    pub size: u32,
    /// Direct data block addresses
    pub addrs: [u32; NDIRECT + 1],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuperBlock {
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

fn write_sp(fsfd: &mut File) {
    let sb = SuperBlock {
        magic: FS_MAGIC,
        size: FSSIZE,
        nblocks: FSSIZE,
        ninodes: NINODES,
        nlog: LOGSIZE,
        logstart: 2,
        inodestart: 2 + LOGSIZE,
        bmapstart: 2 + LOGSIZE + NINODEBLOCKS,
    };
    fsfd.seek(std::io::SeekFrom::Start(BSIZE as u64))
        .unwrap_or_else(|e| {
            eprintln!("cannot seek fs.img: {}", e);
            exit(1);
        });
    let sb_bytes = unsafe {
        std::slice::from_raw_parts(
            &sb as *const SuperBlock as *const u8,
            std::mem::size_of::<SuperBlock>(),
        )
    };
    fsfd.write_all(sb_bytes).unwrap_or_else(|e| {
        eprintln!("cannot write superblock to fs.img: {}", e);
        exit(1);
    });
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} fs.img files ...", args[0]);
        exit(1);
    }
    // int must be 4
    assert_eq!(std::mem::size_of::<u32>(), 4);
    let mut fs = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args[1])
        .unwrap_or_else(|e| {
            eprintln!("cannot open fs.img: {}", e);
            exit(1);
        });
    // let nmeta = 2 + nlog + ninodeblocks + nbitmap;
    fs.set_len((FSSIZE * BSIZE as u32) as u64)
        .unwrap_or_else(|e| {
            eprintln!("cannot set fs.img size: {}", e);
            exit(1);
        });
    write_sp(&mut fs);
    let buf = [0u8; BSIZE];
    for _ in 0..LOGSIZE {
        fs.write_all(&buf).unwrap_or_else(|e| {
            eprintln!("cannot write log to fs.img: {}", e);
            exit(1);
        });
    }
}
