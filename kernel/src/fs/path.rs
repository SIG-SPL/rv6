#![allow(dead_code)]
//! Pathname layer of file system.

use super::inode::{FType, Inode};
use config::fs::ROOTINO;

/// Look up and return the inode for a path name.
/// If parent is true, return the inode for the parent and copy the final
/// path element into name, else return the inode for the last path element.
fn namex(path: &str, nameiparent: bool) -> Option<(Inode, &str)> {
    let mut path = path;
    let mut name = "";
    // relative path or absolute path
    let mut ip = if path.starts_with('/') {
        Inode::get(ROOTINO)
    } else {
        todo!("namex: relative path")
    };
    loop {
        while path.starts_with('/') {
            path = &path[1..];
        }
        if path.is_empty() {
            break;
        }
        let mut next = path;
        while !next.is_empty() && !next.starts_with('/') {
            next = &next[1..];
        }
        name = &path[..path.len() - next.len()];
        path = next;
        let ip_next = ip.dirlookup(name)?;
        ip = ip_next;
        if ip.dinode.typ != FType::Dir {
            break;
        }
    }
    if nameiparent {
        Some((ip, name))
    } else {
        Some((ip, path))
    }
}

/// Look up and return the inode for a path name.
pub fn namei(path: &str) -> Option<Inode> {
    let (inode, _) = namex(path, false)?;
    Some(inode)
}

/// Look up and return the inode for a parent and the final path name element.
pub fn nameiparent(path: &str) -> Option<(Inode, &str)> {
    namex(path, true)
}
