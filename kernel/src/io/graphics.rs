#![allow(dead_code)]

use alloc::vec;
use alloc::vec::Vec;

use super::virtio::gpu;
use crate::console::{CtrlChar, EscapeCode, InputMode};
use crate::sync::SpinLock;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;

const ASCII_CNT: usize = 95;
const BITMAP_SIZE: usize = (CHAR_WIDTH / 8) * CHAR_HEIGHT * ASCII_CNT;

const SIMSUN_FONT: [u8; BITMAP_SIZE] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x48, 0x24, 0x24, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x48, 0x48, 0x48, 0x7e, 0x24, 0x24, 0x24, 0x7e, 0x24, 0x24, 0x24, 0x00, 0x00, 0x00,
    0x00, 0x10, 0x3c, 0x52, 0x52, 0x12, 0x1c, 0x30, 0x50, 0x50, 0x52, 0x52, 0x3c, 0x10, 0x10, 0x00,
    0x00, 0x00, 0x22, 0x25, 0x15, 0x15, 0x0d, 0x2a, 0x58, 0x54, 0x54, 0x52, 0x22, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x0c, 0x12, 0x12, 0x12, 0x0a, 0x76, 0x25, 0x29, 0x19, 0x91, 0x6e, 0x00, 0x00, 0x00,
    0x06, 0x04, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x40, 0x20, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x10, 0x20, 0x40, 0x00, 0x00,
    0x02, 0x04, 0x08, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08, 0x04, 0x02, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x08, 0x08, 0x6b, 0x1c, 0x1c, 0x6b, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x10, 0x10, 0x10, 0xfe, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x04, 0x04, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x00,
    0x00, 0x40, 0x20, 0x20, 0x20, 0x10, 0x10, 0x08, 0x08, 0x08, 0x04, 0x04, 0x02, 0x02, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x24, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x24, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x10, 0x1c, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x7c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x40, 0x20, 0x10, 0x08, 0x04, 0x42, 0x7e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x42, 0x42, 0x40, 0x20, 0x18, 0x20, 0x40, 0x42, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x30, 0x30, 0x28, 0x24, 0x24, 0x22, 0xfe, 0x20, 0x20, 0xf8, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x02, 0x02, 0x02, 0x1e, 0x22, 0x40, 0x40, 0x42, 0x22, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x24, 0x02, 0x02, 0x3a, 0x46, 0x42, 0x42, 0x42, 0x44, 0x38, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x42, 0x20, 0x20, 0x10, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x24, 0x18, 0x24, 0x42, 0x42, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1c, 0x22, 0x42, 0x42, 0x42, 0x62, 0x5c, 0x40, 0x40, 0x24, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x00,
    0x00, 0x00, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x42, 0x42, 0x46, 0x20, 0x10, 0x10, 0x10, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1c, 0x22, 0x5a, 0x55, 0x55, 0x55, 0x55, 0x55, 0x3a, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x08, 0x18, 0x14, 0x14, 0x24, 0x3c, 0x22, 0x42, 0x42, 0xe7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1f, 0x22, 0x22, 0x22, 0x1e, 0x22, 0x42, 0x42, 0x42, 0x22, 0x1f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7c, 0x42, 0x42, 0x01, 0x01, 0x01, 0x01, 0x01, 0x42, 0x22, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1f, 0x22, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x22, 0x1f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3f, 0x42, 0x12, 0x12, 0x1e, 0x12, 0x12, 0x02, 0x42, 0x42, 0x3f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3f, 0x42, 0x12, 0x12, 0x1e, 0x12, 0x12, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x22, 0x22, 0x01, 0x01, 0x01, 0x71, 0x21, 0x22, 0x22, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xe7, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7c, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x11, 0x0f, 0x00,
    0x00, 0x00, 0x77, 0x22, 0x12, 0x0a, 0x0e, 0x0a, 0x12, 0x12, 0x22, 0x22, 0x77, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x07, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x42, 0x7f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x77, 0x36, 0x36, 0x36, 0x36, 0x36, 0x2a, 0x2a, 0x2a, 0x2a, 0x6b, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xe3, 0x46, 0x46, 0x4a, 0x4a, 0x52, 0x52, 0x52, 0x62, 0x62, 0x47, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x22, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3f, 0x42, 0x42, 0x42, 0x42, 0x3e, 0x02, 0x02, 0x02, 0x02, 0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1c, 0x22, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x4d, 0x32, 0x1c, 0x60, 0x00, 0x00,
    0x00, 0x00, 0x3f, 0x42, 0x42, 0x42, 0x3e, 0x12, 0x12, 0x22, 0x22, 0x42, 0xc7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7c, 0x42, 0x42, 0x02, 0x04, 0x18, 0x20, 0x40, 0x42, 0x42, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7f, 0x49, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xe7, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xe7, 0x42, 0x42, 0x22, 0x24, 0x24, 0x14, 0x14, 0x18, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x6b, 0x2a, 0x2a, 0x2a, 0x2a, 0x2a, 0x36, 0x14, 0x14, 0x14, 0x14, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xe7, 0x42, 0x24, 0x24, 0x18, 0x18, 0x18, 0x24, 0x24, 0x42, 0xe7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x77, 0x22, 0x22, 0x14, 0x14, 0x08, 0x08, 0x08, 0x08, 0x08, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x21, 0x20, 0x10, 0x10, 0x08, 0x04, 0x04, 0x42, 0x42, 0x3f, 0x00, 0x00, 0x00,
    0x78, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x78, 0x00, 0x00,
    0x00, 0x02, 0x04, 0x04, 0x04, 0x08, 0x08, 0x08, 0x10, 0x10, 0x20, 0x20, 0x20, 0x40, 0x40, 0x00,
    0x1e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1e, 0x00, 0x00,
    0x18, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00,
    0x06, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x22, 0x30, 0x2c, 0x22, 0x32, 0x6c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x03, 0x02, 0x02, 0x1a, 0x26, 0x42, 0x42, 0x42, 0x26, 0x1a, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x44, 0x02, 0x02, 0x02, 0x44, 0x38, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x60, 0x40, 0x40, 0x7c, 0x42, 0x42, 0x42, 0x42, 0x62, 0xdc, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x7e, 0x02, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x30, 0x48, 0x08, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x22, 0x22, 0x1c, 0x02, 0x3c, 0x42, 0x42, 0x3c, 0x00,
    0x00, 0x00, 0x00, 0x03, 0x02, 0x02, 0x3a, 0x46, 0x42, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x0c, 0x0c, 0x00, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x30, 0x30, 0x00, 0x00, 0x38, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x1e, 0x00,
    0x00, 0x00, 0x00, 0x03, 0x02, 0x02, 0x72, 0x12, 0x0a, 0x0e, 0x12, 0x22, 0x77, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x92, 0x92, 0x92, 0x92, 0x92, 0xb7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x46, 0x42, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x26, 0x42, 0x42, 0x42, 0x26, 0x1a, 0x02, 0x07, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x64, 0x42, 0x42, 0x42, 0x64, 0x58, 0x40, 0xe0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x4c, 0x04, 0x04, 0x04, 0x04, 0x1f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x42, 0x02, 0x3c, 0x40, 0x42, 0x3e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x48, 0x30, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x42, 0x42, 0x42, 0x42, 0x62, 0xdc, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x22, 0x22, 0x14, 0x14, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0x91, 0x52, 0x5a, 0x2a, 0x24, 0x24, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x24, 0x18, 0x18, 0x18, 0x24, 0x76, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0x42, 0x24, 0x24, 0x18, 0x18, 0x08, 0x08, 0x06, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x22, 0x10, 0x08, 0x08, 0x44, 0x7e, 0x00, 0x00, 0x00,
    0xc0, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x10, 0x20, 0x20, 0x20, 0x20, 0x20, 0xc0, 0x00, 0x00,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x01,
    0x03, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x03, 0x00, 0x00,
    0x5a, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const SIMHEI_FONT: [u8; BITMAP_SIZE] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x08, 0x08, 0x08, 0x08, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x24, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x44, 0x24, 0x24, 0xff, 0x24, 0x24, 0x24, 0xff, 0x22, 0x12, 0x12, 0x00, 0x00, 0x00,
    0x00, 0x08, 0x3c, 0x2e, 0x6a, 0x0e, 0x0c, 0x38, 0x68, 0x6a, 0x6a, 0x2e, 0x1c, 0x08, 0x00, 0x00,
    0x00, 0x00, 0x26, 0x25, 0x15, 0x1d, 0x16, 0x68, 0x58, 0x54, 0x54, 0x52, 0x62, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1c, 0x34, 0x34, 0x14, 0x0c, 0x0e, 0x4a, 0x53, 0x63, 0xf6, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x40, 0x20, 0x30, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x20, 0x20, 0x40, 0x00, 0x00,
    0x00, 0x02, 0x04, 0x04, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x04, 0x06, 0x03, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x18, 0x7e, 0x18, 0x3c, 0x5a, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0xff, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x06, 0x02, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x40, 0x20, 0x20, 0x10, 0x10, 0x08, 0x08, 0x04, 0x04, 0x02, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x26, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x62, 0x34, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x10, 0x18, 0x1e, 0x1a, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x60, 0x20, 0x30, 0x10, 0x08, 0x04, 0x7e, 0x7e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x60, 0x30, 0x30, 0x60, 0x40, 0x42, 0x26, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x30, 0x38, 0x28, 0x24, 0x26, 0x22, 0xff, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7c, 0x06, 0x02, 0x0a, 0x3e, 0x62, 0x40, 0x40, 0x63, 0x36, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x10, 0x18, 0x08, 0x0c, 0x3e, 0x46, 0xc2, 0xc2, 0x42, 0x66, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x40, 0x60, 0x20, 0x30, 0x10, 0x18, 0x08, 0x08, 0x0c, 0x0c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x62, 0x42, 0x62, 0x3e, 0x3e, 0x42, 0x43, 0x42, 0x66, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x62, 0x43, 0x43, 0x63, 0x36, 0x3c, 0x10, 0x18, 0x08, 0x0c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x08, 0x00, 0x00,
    0x00, 0x00, 0x40, 0x20, 0x10, 0x08, 0x06, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x20, 0x10, 0x08, 0x06, 0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x62, 0x60, 0x30, 0x10, 0x08, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x42, 0x79, 0x69, 0x65, 0x55, 0x55, 0x55, 0x29, 0x02, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x18, 0x18, 0x3c, 0x24, 0x24, 0x3e, 0x66, 0x42, 0x42, 0xc3, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3e, 0x62, 0x42, 0x42, 0x3e, 0x3e, 0x42, 0x42, 0x42, 0x7e, 0x1e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x42, 0x02, 0x02, 0x42, 0x42, 0x46, 0x6c, 0x38, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1e, 0x32, 0x62, 0x42, 0x42, 0x42, 0x42, 0x42, 0x62, 0x3e, 0x0e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x02, 0x02, 0x02, 0x7e, 0x7e, 0x02, 0x02, 0x02, 0x7e, 0x7e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x02, 0x02, 0x02, 0x02, 0x3e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x42, 0x02, 0x72, 0x72, 0x42, 0x46, 0x6c, 0x58, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x7e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x42, 0x62, 0x3e, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x62, 0x22, 0x12, 0x1a, 0x0e, 0x1e, 0x12, 0x32, 0x22, 0x62, 0xc2, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x7e, 0x7e, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x76, 0x5e, 0x5a, 0x5a, 0x5a, 0x5a, 0x4a, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x42, 0x46, 0x46, 0x4e, 0x4a, 0x5a, 0x52, 0x72, 0x62, 0x62, 0x62, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x42, 0x43, 0x43, 0x43, 0x42, 0x62, 0x26, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3e, 0x62, 0x42, 0x42, 0x42, 0x7e, 0x0e, 0x02, 0x02, 0x02, 0x02, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x42, 0x42, 0x43, 0x43, 0x43, 0x52, 0x72, 0x26, 0x78, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3e, 0x62, 0x42, 0x42, 0x62, 0x3e, 0x12, 0x32, 0x22, 0x62, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x3c, 0x66, 0x62, 0x06, 0x0c, 0x38, 0x60, 0x42, 0x42, 0x66, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x43, 0x42, 0x62, 0x66, 0x26, 0x24, 0x34, 0x1c, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xd9, 0x5b, 0x5b, 0x5a, 0x5a, 0x56, 0x56, 0x66, 0x66, 0x26, 0x26, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x62, 0x26, 0x24, 0x1c, 0x18, 0x18, 0x1c, 0x34, 0x26, 0x62, 0x43, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x43, 0x62, 0x26, 0x34, 0x1c, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7e, 0x60, 0x20, 0x30, 0x10, 0x08, 0x0c, 0x04, 0x06, 0x7e, 0x7e, 0x00, 0x00, 0x00,
    0x78, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x78, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x04, 0x04, 0x04, 0x08, 0x08, 0x10, 0x10, 0x10, 0x20, 0x20, 0x40, 0x40, 0x00,
    0x1e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1e, 0x00, 0x00,
    0x18, 0x34, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00,
    0x0c, 0x18, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x62, 0x70, 0x6e, 0x62, 0x72, 0x5c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x02, 0x02, 0x02, 0x3e, 0x62, 0x42, 0x42, 0x62, 0x66, 0x1a, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x62, 0x02, 0x02, 0x42, 0x66, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x40, 0x40, 0x40, 0x40, 0x7e, 0x62, 0x42, 0x42, 0x62, 0x66, 0x5c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x62, 0x7e, 0x02, 0x42, 0x66, 0x38, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x78, 0x08, 0x08, 0x08, 0x7e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x22, 0x22, 0x3c, 0x02, 0x3e, 0x62, 0x42, 0x3e, 0x00,
    0x00, 0x00, 0x02, 0x02, 0x02, 0x02, 0x7a, 0x46, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x30, 0x30, 0x00, 0x00, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x32, 0x1e, 0x00,
    0x00, 0x00, 0x02, 0x02, 0x02, 0x02, 0x32, 0x1a, 0x1e, 0x16, 0x22, 0x62, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x46, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x62, 0x42, 0x42, 0x42, 0x66, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x62, 0x42, 0x42, 0x62, 0x66, 0x1a, 0x02, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x62, 0x42, 0x42, 0x62, 0x66, 0x5c, 0x40, 0x40, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x66, 0x06, 0x38, 0x42, 0x66, 0x3c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x3f, 0x08, 0x08, 0x08, 0x08, 0x48, 0x70, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x62, 0x76, 0x5c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x66, 0x24, 0x24, 0x1c, 0x18, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0x5b, 0x5a, 0x56, 0x66, 0x26, 0x24, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x34, 0x18, 0x18, 0x3c, 0x26, 0x42, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x66, 0x24, 0x24, 0x1c, 0x18, 0x18, 0x08, 0x0e, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x30, 0x10, 0x08, 0x04, 0x06, 0x7e, 0x00, 0x00, 0x00,
    0x60, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x10, 0x30, 0x30, 0x30, 0x30, 0x30, 0x20, 0x00, 0x00,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x03,
    0x06, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00,
    0x4e, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

lazy_static! {
    pub static ref TEXT_BUFFER: SpinLock<TextBuffer> =
        SpinLock::new(TextBuffer::new(Font::default()), "TextBufferLock");
}

#[derive(Default, Clone, Copy)]
#[allow(unused)]
pub enum Font {
    #[default]
    SimSun,
    SimHei,
}

pub struct TextBuffer {
    /// Text buffer
    pub chars: Vec<char>,
    /// Current text cursor position: (line, column)
    /// - line: 0-based line index
    /// - column: 0-based column index
    /// - (0, 0) is the top-left corner
    pub cursor: (u32, u32),
    /// Maximum number of chars per line
    pub line_nchars: u32,
    /// Maximum number of lines
    pub max_lines: u32,
    /// Current font
    pub font: Font,
    pub input_mode: InputMode,
}

/// Returns the index of a displable ascii char in the font array
/// Unhandled control chars are mapped to the space char
fn ascii_to_index(c: char) -> usize {
    match c as u8 {
        0x20..=0x7E => (c as u8 - 0x20) as usize,
        _ => 0,
    }
}

/// Returns the bitmap of a font
fn get_font_bitmap(font: Font) -> &'static [u8; BITMAP_SIZE] {
    match font {
        Font::SimSun => &SIMSUN_FONT,
        Font::SimHei => &SIMHEI_FONT,
    }
}

/// Draws an ascii char at the given position
/// - px: starting x position
/// - py: starting y position
fn draw_ascii(px: u32, py: u32, c: char, font: Font) {
    let index = ascii_to_index(c);
    let bitmap = get_font_bitmap(font);
    for (i, byte) in bitmap[index * CHAR_HEIGHT..(index + 1) * CHAR_HEIGHT]
        .iter()
        .enumerate()
    {
        for bit in 0..8 {
            if byte & (1 << bit) != 0 {
                let x = px + bit;
                let y = py + i as u32;
                gpu::set_pixel(x, y, 0xff, 0xff, 0xff, 0).unwrap();
            }
        }
    }
}

fn clear_ascii(px: u32, py: u32) {
    for x in px..px + CHAR_WIDTH as u32 {
        for y in py..py + CHAR_HEIGHT as u32 {
            gpu::set_pixel(x, y, 0, 0, 0, 0).unwrap();
        }
    }
}

impl TextBuffer {
    pub fn new(font: Font) -> Self {
        let (width, height) = gpu::get_resolution();
        let line_nchars = width / CHAR_WIDTH as u32;
        let max_lines = height / CHAR_HEIGHT as u32;
        Self {
            chars: vec![' '; line_nchars as usize * max_lines as usize],
            cursor: (0, 0),
            line_nchars,
            max_lines,
            font,
            input_mode: InputMode::default(),
        }
    }

    fn index(&self) -> usize {
        (self.cursor.0 * self.line_nchars + self.cursor.1) as usize
    }

    fn scroll(&mut self) {
        for i in 0..self.max_lines - 1 {
            // clear & flush the upper line first to avoid font corruption
            for j in 0..self.line_nchars {
                let idx = (i * self.line_nchars + j) as usize;
                self.chars[idx] = ' ';
            }
            self.flush_line(i);
            // move lines up by one
            for j in 0..self.line_nchars {
                let idx = (i * self.line_nchars + j) as usize;
                let idx2 = ((i + 1) * self.line_nchars + j) as usize;
                self.chars[idx] = self.chars[idx2];
            }
        }
        // clear & flush the last line
        for j in 0..self.line_nchars {
            let idx = ((self.max_lines - 1) * self.line_nchars + j) as usize;
            self.chars[idx] = ' ';
        }
        self.flush_line(self.max_lines - 1);
        // redirect the cursor
        self.cursor.0 -= 1;
    }

    pub fn putc(&mut self, c: char) {
        match self.input_mode {
            InputMode::Insert => self.putc_normal(c),
            InputMode::Replace => {
                self.putc_normal(CtrlChar::DEL);
                self.putc_normal(c);
            }
            _ => self.putc_escape(c),
        }
    }

    fn putc_escape(&mut self, c: char) {
        match self.input_mode {
            InputMode::EscapeState1 => {
                assert_eq!(c, EscapeCode::START);
                self.input_mode = InputMode::EscapeState2;
            }
            InputMode::EscapeState2 => {
                match c {
                    EscapeCode::START => unreachable!(),
                    EscapeCode::VK_UP => {
                        println!("up");
                    }
                    EscapeCode::VK_DOWN => {
                        println!("down");
                    }
                    EscapeCode::VK_RIGHT => {
                        println!("right");
                    }
                    EscapeCode::VK_LEFT => {
                        println!("left");
                    }
                    _ => (),
                }
                self.input_mode = InputMode::Insert;
            }
            _ => unreachable!(),
        }
    }

    fn putc_normal(&mut self, c: char) {
        match c {
            CtrlChar::CR | CtrlChar::LF => {
                if self.cursor.0 + 1 >= self.max_lines {
                    self.scroll();
                }
                self.cursor.0 += 1;
                self.cursor.1 = 0;
                self.flush();
            }
            CtrlChar::BS | CtrlChar::DEL => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    let idx = self.index();
                    self.chars[idx] = ' ';
                    self.flush_current_line();
                }
            }
            CtrlChar::HT => {
                for _ in 0..4 {
                    if self.cursor.1 + 1 < self.line_nchars {
                        self.putc(' ');
                    } else {
                        break;
                    }
                }
            }
            CtrlChar::ESC => {
                self.input_mode = InputMode::EscapeState1;
            }
            _ => {
                if self.cursor.1 + 1 >= self.line_nchars {
                    self.putc('\n');
                }
                let idx = self.index();
                self.chars[idx] = c;
                self.cursor.1 += 1;
                self.flush_current_line();
            }
        }
    }

    /// Flush the whole text buffer
    /// - This function does not clear spaces
    /// - This is time-consuming!
    /// - We don't flush GPU in TextBuffer because GPU will be flushed
    ///    every time the clock ticks, and is much faster
    pub fn flush(&self) {
        for (i, c) in self.chars.iter().enumerate() {
            let x = (i as u32 % self.line_nchars) * CHAR_WIDTH as u32;
            let y = (i as u32 / self.line_nchars) * CHAR_HEIGHT as u32;
            draw_ascii(x, y, *c, self.font);
        }
    }

    /// Flush a single line
    /// This function clears spaces and avoids font corruption
    pub fn flush_line(&self, ln: u32) {
        for j in 0..self.line_nchars {
            let idx = (ln * self.line_nchars + j) as usize;
            let x = j * CHAR_WIDTH as u32;
            let y = ln * CHAR_HEIGHT as u32;
            match self.chars[idx] {
                ' ' => clear_ascii(x, y),
                _ => draw_ascii(x, y, self.chars[idx], self.font),
            }
        }
    }

    fn flush_current_line(&self) {
        self.flush_line(self.cursor.0);
    }
}
