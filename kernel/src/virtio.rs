use core::{
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

use fdt::{node::FdtNode, Fdt};
use virtio_drivers::{
    transport::{
        mmio::{MmioError, MmioTransport, VirtIOHeader},
        DeviceType, Transport,
    },
    BufferDirection, Hal, PhysAddr, PAGE_SIZE,
};

pub fn init(dtb_pa: usize) {
    info!("DTB physical address @ {:#x}", dtb_pa);
    // parse flattened device tree from memory
    let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).unwrap() };
    for node in fdt.all_nodes() {
        // detect virtio-mmio devices by compatible string
        if let Some(compatible) = node.compatible() {
            if compatible.all().any(|s| s == "virtio,mmio") {
                virtio_mmio_init(node);
            }
        }
    }
}

fn virtio_mmio_init(node: FdtNode) {
    if let Some(reg) = node.reg().and_then(|mut r| r.next()) {
        let pa = reg.starting_address as usize;
        // map the device into kernel address space
        // we don't have virtual memory yet, so we just map it to the same address
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
                    _ => warn!("Device type {:?} not supported", transport.device_type()),
                }
            }
        }
    }
}

pub mod block {

    use virtio_drivers::{device::blk::VirtIOBlk, transport::mmio::MmioTransport, Result};

    use super::HalImpl;

    /// The global block device. We support only one block device for now.
    static mut DEVICE: Option<VirtIOBlk<HalImpl, MmioTransport>> = None;

    pub fn init(transport: MmioTransport) {
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
    pub fn write(block_id: usize, buf: &[u8]) -> Result {
        unsafe {
            if let Some(ref mut blk) = DEVICE {
                blk.write_block(block_id, buf)?;
            }
        }
        Ok(())
    }

    /// Wrapper function for reading from the block device.
    pub fn read(block_id: usize, buf: &mut [u8]) -> Result {
        unsafe {
            if let Some(ref mut blk) = DEVICE {
                blk.read_block(block_id, buf)?;
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
    static ref DMA_PA: AtomicUsize = AtomicUsize::new(ekernel as usize);
}

fn virt_to_phys(va: usize) -> PhysAddr {
    va
}

#[allow(unused_variables)]
unsafe impl Hal for HalImpl {
    fn dma_alloc(pages: usize, direction: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let pa = DMA_PA.fetch_add(pages * PAGE_SIZE, Ordering::SeqCst);
        let va = NonNull::new(pa as _).unwrap();
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

    unsafe fn unshare(paddr: PhysAddr, buffer: NonNull<[u8]>, direction: BufferDirection) {
        ()
    }
}
