pub(crate) mod graphics;
pub(crate) mod stdio;
pub mod virtio;

pub use stdio::{Stdin, Stdout, STDIN};
pub use virtio::init;
