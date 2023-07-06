use super::inode::Inode;

/// File is a wrapper around of inode, ...
pub enum File {
    Inode { inode: Inode, off: usize },
}
