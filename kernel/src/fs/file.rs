use super::nameiparent;
use super::Inode;

/// File is a wrapper around of inode, device, pipe or None
pub enum File {
    Inode { inode: Inode, off: usize },
    None,
}

impl File {
    pub fn open(path: &str, _omode: u32) -> Option<Self> {
        let (mut pinode, name) = nameiparent(path)?;
        match pinode.dirlookup(name) {
            Some(inode) => Some(Self::Inode { inode, off: 0 }),
            None => {
                // create a new file
                let inode = Inode::new(super::FType::File, 0, 0);
                pinode.dirlink(name, inode.inum)?;
                Some(Self::Inode { inode, off: 0 })
            }
        }
    }
}
