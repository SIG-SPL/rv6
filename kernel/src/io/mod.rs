pub(crate) mod graphics;
pub(crate) mod stdio;
pub(crate) mod virtio;

pub use stdio::{STDIN, Stdout, Stdin};
pub use virtio::init;
