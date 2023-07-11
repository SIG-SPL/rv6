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

/// On-disk inode structure copy from rv6 kernel/inode.rs
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
    // write root inode
    let dinode = DInode {
        typ: FType::Dir,
        major: 0,
        minor: 0,
        nlink: 1,
        size: 0,
        addrs: [0; NDIRECT + 1],
    };

    let dinode_bytes = unsafe {
        std::slice::from_raw_parts(
            &dinode as *const DInode as *const u8,
            std::mem::size_of::<DInode>(),
        )
    };

    let root_start_address =
        (INDOE_START) * BSIZE + ROOTINO as usize * std::mem::size_of::<DInode>();
    fs.seek(std::io::SeekFrom::Start(root_start_address as u64))
        .unwrap_or_else(|e| {
            eprintln!("cannot seek fs.img: {}", e);
            exit(1);
        });
    fs.write_all(dinode_bytes).unwrap_or_else(|e| {
        eprintln!("cannot write root inode to fs.img: {}", e);
        exit(1);
    });
    // write bitmap
    let bitmap_start_address = INODE_BITMAP_START * BSIZE;
    fs.seek(std::io::SeekFrom::Start(bitmap_start_address as u64))
        .unwrap_or_else(|e| {
            eprintln!("cannot seek fs.img: {}", e);
            exit(1);
        });
    // write inode bitmap
    // root inode is used
    // so the second bit is set to 1
    // and the first bit is set to 1 since we don't use it
    let mut bitmap = [0u8; BSIZE];
    bitmap[0] = 0b0000_0011;
    fs.write_all(&bitmap).unwrap_or_else(|e| {
        eprintln!("cannot write bitmap to fs.img: {}", e);
        exit(1);
    });
    // write initial files to root dir
    // for i in 2..args.len() {
    //     let path = format!("../target/riscv64gc-unknown-none-elf/release/{}", args[i]);
    //     if std::fs::metadata(&path).is_err() {
    //         eprintln!("{} does not exist", path);
    //         continue;
    //     }
    //     let data = std::fs::read(&path).unwrap_or_else(|e| {
    //         eprintln!("cannot read {}: {}", path, e);
    //         exit(1);
    //     });
    //     let mut name = args[i].clone();
    // }
}
