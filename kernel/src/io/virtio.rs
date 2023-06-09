#![allow(clippy::result_unit_err)]

use core::{
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

use config::vm::PA2VA_OFFSET;
use fdt::{node::FdtNode, Fdt};
use virtio_drivers::{
    transport::{
        mmio::{MmioError, MmioTransport, VirtIOHeader},
        DeviceType, Transport,
    },
    BufferDirection, Hal, PhysAddr, PAGE_SIZE,
};

/// Virtual memory should be initialized before calling this function.
pub fn init(dtb_pa: usize) {
    // DTB lies in riscv_virt_board.ram, so we add the offset to get va
    let dtb_va = dtb_pa + PA2VA_OFFSET;
    info!("Device tree blob @ {:#x}", dtb_va);
    // parse flattened device tree from memory
    let fdt = unsafe { Fdt::from_ptr(dtb_va as *const u8).unwrap() };
    for node in fdt.all_nodes() {
        // detect virtio-mmio devices by compatible string
        if let Some(compatible) = node.compatible() {
            if compatible.all().any(|s| s == "virtio,mmio") {
                virtio_mmio_init(node);
            }
        }
    }
    info!("All MMIO devices probed and initialized")
}

fn virtio_mmio_init(node: FdtNode) {
    if let Some(reg) = node.reg().and_then(|mut r| r.next()) {
        let pa = reg.starting_address as usize;
        // We map MMIO pa to equal va in S mode
        let va = pa;
        // get the device header, validate it and initialize the device
        let header = NonNull::new(va as *mut VirtIOHeader).unwrap();
        match unsafe { MmioTransport::new(header) } {
            Err(e) => match e {
                MmioError::ZeroDeviceId => (),
                _ => error!("Failed to initialize virtio-mmio device: {:?}", e),
            },
            Ok(transport) => {
                info!(
                    "Detected {} with vendor id {:#X}, device type {:?}, version {:?}",
                    node.name,
                    transport.vendor_id(),
                    transport.device_type(),
                    transport.version()
                );
                match transport.device_type() {
                    DeviceType::Block => block::init(transport),
                    DeviceType::GPU => gpu::init(transport),
                    _ => warn!("Device type {:?} not supported", transport.device_type()),
                }
            }
        }
    }
}

#[allow(unused)]
pub mod block {

    use virtio_drivers::{device::blk::VirtIOBlk, transport::mmio::MmioTransport, Result};

    use crate::sync::SpinLock;

    use super::HalImpl;

    /// The global block device. We support only one block device for now.
    static mut DEVICE: Option<VirtIOBlk<HalImpl, MmioTransport>> = None;
    static mut SPINLOCK: SpinLock<()> = SpinLock::new((), "VirtIOBlkLock");

    pub(super) fn init(transport: MmioTransport) {
        if unsafe { DEVICE.is_some() } {
            warn!("Only one block device is supported");
            return;
        }
        match VirtIOBlk::<HalImpl, MmioTransport>::new(transport) {
            Err(e) => error!("Failed to initialize virtio block device: {:?}", e),
            Ok(blk) => {
                info!("Initialized virtio block device");
                unsafe {
                    DEVICE = Some(blk);
                }
            }
        }
    }

    /// Wrapper function for writing to the block device.
    /// The funtion blocks the thread until the write is finished.
    /// Only one thread can write to the block device at a time.
    pub fn write(block_id: usize, buf: &[u8]) -> Result {
        unsafe {
            let _lock = SPINLOCK.lock();
            if let Some(ref mut blk) = DEVICE {
                blk.write_block(block_id, buf)?;
            }
        }
        Ok(())
    }

    /// Wrapper function for reading from the block device.
    /// The funtion blocks the thread until the read is finished.
    /// Only one thread can read from the block device at a time.
    pub fn read(block_id: usize, buf: &mut [u8]) -> Result {
        unsafe {
            let _lock = SPINLOCK.lock();
            if let Some(ref mut blk) = DEVICE {
                blk.read_block(block_id, buf)?;
            }
        }
        Ok(())
    }

    /// Capacity of the block device in sectors (512 bytes per sector)
    pub fn capacity() -> u64 {
        unsafe {
            let _lock = SPINLOCK.lock();
            if let Some(ref mut blk) = DEVICE {
                blk.capacity()
            } else {
                0
            }
        }
    }
}

#[allow(unused)]
pub mod gpu {

    use virtio_drivers::{device::gpu::VirtIOGpu, transport::mmio::MmioTransport};

    type Result<T = ()> = core::result::Result<T, ()>;

    use crate::sync::SpinLock;

    use super::HalImpl;

    static mut DEVICE: Option<VirtIOGpu<HalImpl, MmioTransport>> = None;
    static mut FRAMEBUFFER: Option<&mut [u8]> = None;

    /// Global spinlock for the virtio gpu device.
    /// The lock blocks the caller if another thread is calling flush or set_pixel.
    static mut SPINLOCK: SpinLock<()> = SpinLock::new((), "VirtIOGpuLock");

    static mut WIDTH: u32 = 0;
    static mut HEIGHT: u32 = 0;

    pub(super) fn init(transport: MmioTransport) {
        match VirtIOGpu::<HalImpl, MmioTransport>::new(transport) {
            Err(e) => error!("Failed to initialize virtio gpu device: {:?}", e),
            Ok(mut gpu) => {
                match gpu.resolution() {
                    Ok((w, h)) => {
                        info!("Initialized virtio gpu device with resolution {}x{}", w, h);
                        unsafe {
                            WIDTH = w;
                            HEIGHT = h;
                        }
                    }
                    Err(e) => error!("Failed to get resolution of virtio gpu device: {:?}", e),
                }
                unsafe {
                    DEVICE = Some(gpu);
                    if let Some(ref mut gpu) = DEVICE {
                        match gpu.setup_framebuffer() {
                            Ok(fb) => {
                                info!("Initialized virtio gpu framebuffer");
                                FRAMEBUFFER = Some(fb);
                            }
                            Err(e) => error!("Failed to setup virtio gpu framebuffer: {:?}", e),
                        }
                    }
                }
            }
        }
    }

    pub fn flush() -> Result {
        unsafe {
            let _lock = SPINLOCK.lock();
            if let Some(ref mut gpu) = DEVICE {
                return match gpu.flush() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
            }
        }
        Ok(())
    }

    pub fn get_resolution() -> (u32, u32) {
        (unsafe { WIDTH }, unsafe { HEIGHT })
    }

    pub fn get_width() -> u32 {
        unsafe { WIDTH }
    }

    pub fn get_height() -> u32 {
        unsafe { HEIGHT }
    }

    pub fn set_pixel(px: u32, py: u32, r: u8, g: u8, b: u8, alpha: u8) -> Result {
        unsafe {
            let _lock = SPINLOCK.lock();
            if let Some(ref mut fb) = FRAMEBUFFER {
                let idx = ((py * get_width() + px) * 4) as usize;
                (*fb)[idx] = r;
                (*fb)[idx + 1] = g;
                (*fb)[idx + 2] = b;
                (*fb)[idx + 3] = alpha;
            }
        }
        Ok(())
    }
}
struct HalImpl;

extern "C" {
    fn ekernel();
}

lazy_static! {
    static ref DMA_PA: AtomicUsize = AtomicUsize::new(ekernel as usize - PA2VA_OFFSET);
}

fn virt_to_phys(va: usize) -> PhysAddr {
    va - PA2VA_OFFSET
}

#[allow(unused_variables)]
unsafe impl Hal for HalImpl {
    fn dma_alloc(pages: usize, direction: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let pa = DMA_PA.fetch_add(pages * PAGE_SIZE, Ordering::SeqCst);
        let va = NonNull::new((pa + PA2VA_OFFSET) as _).unwrap();
        (pa, va)
    }

    unsafe fn dma_dealloc(paddr: PhysAddr, vaddr: NonNull<u8>, pages: usize) -> i32 {
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, size: usize) -> NonNull<u8> {
        NonNull::new(paddr as _).unwrap()
    }

    unsafe fn share(buffer: NonNull<[u8]>, direction: BufferDirection) -> PhysAddr {
        let va = buffer.as_ptr() as *mut u8 as usize;
        virt_to_phys(va)
    }

    unsafe fn unshare(paddr: PhysAddr, buffer: NonNull<[u8]>, direction: BufferDirection) {}
}
