//! Sv39 Mode Virtual Memory Management

use alloc::boxed::Box;
use config::{layout::*, vm::*};

use crate::mm::allocator::alloc_page;

lazy_static! {
    static ref ROOT_PT: Box<PageTable> = Box::new(PageTable::new());
}

/// Global allocator should be initialized before calling this function.
pub fn init() {
    extern "C" {
        fn stext();
        fn srodata();
        fn sdata();
    }

    let pta = ROOT_PT.as_ref() as *const PageTable as usize;

    let stxt_pa = PHY_START + OPENSBI_SIZE;
    let txt_len = srodata as usize - stext as usize;

    let srod_pa = stxt_pa + txt_len;
    let rod_len = sdata as usize - srodata as usize;

    let rest_pa = srod_pa + rod_len;
    let rest_len = PHY_STOP - rest_pa;

    kvmmap(pta, PLIC_BASE, PLIC_BASE, PLIC_MMAP_SIZE, PTE_R | PTE_W);
    kvmmap(pta, MMIO_BASE, MMIO_BASE, MMIO_MMAP_SIZE, PTE_R | PTE_W);
    kvmmap(pta, stext as usize, stxt_pa, txt_len, PTE_R | PTE_X);
    kvmmap(pta, srodata as usize, srod_pa, rod_len, PTE_R);
    kvmmap(pta, sdata as usize, rest_pa, rest_len, PTE_R | PTE_W);

    ROOT_PT.flush();
    info!("Initialized MMU, mode: Sv39, root page table @ 0x{:x}", pta);
}

/// Page Table
/// - All PTEs fit in one page
#[repr(align(4096), C)]
pub struct PageTable {
    ent: [PageTableEntry; 512],
}

impl PageTable {
    const fn new() -> Self {
        Self {
            ent: [PageTableEntry::default(); 512],
        }
    }

    fn flush(&self) {
        use riscv::register::satp;

        unsafe {
            satp::set(
                satp::Mode::Sv39,
                0,
                (self as *const PageTable as usize - PA2VA_OFFSET) >> PGSHIFT,
            );
            riscv::asm::sfence_vma_all();
        }
    }
}

impl core::ops::Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ent[index]
    }
}

impl core::ops::IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ent[index]
    }
}

/// Page Table Entry (Sv39)
/// - 64 bits
#[repr(align(8), C)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    const fn default() -> Self {
        Self { bits: 0 }
    }

    /// Link the PTE to the next level page table
    /// - next: The virtual address of the next level page table
    fn link(&mut self, next: usize) {
        let ppn = (next - PA2VA_OFFSET) >> PGSHIFT;
        self.bits = (ppn << PTE_SHIFT) | PTE_V;
    }

    fn set_pa(&mut self, pa: usize, flag: usize) {
        let ppn = pa >> PGSHIFT;
        self.bits = (ppn << PTE_SHIFT) | flag | PTE_V;
    }

    #[inline(always)]
    fn is(&self, flag: usize) -> bool {
        self.bits & flag != 0
    }

    /// Extract the physical page number from the PTE.
    #[inline(always)]
    fn ppn(&self) -> usize {
        self.bits >> PTE_SHIFT
    }

    /// Extract the virtual address the PTE points to.
    #[inline(always)]
    fn va(&self) -> usize {
        (self.ppn() << PGSHIFT) + PA2VA_OFFSET
    }
}

/// Walk through the page table and get the last level PTE for a virtual address.
/// The function creates new entries if needed.
/// - pta: Virtual address of the page table
/// - va: Virtual address to be processed
/// - level: The level of the page table
///     - lv3 pt: 2 == Accessing PPN\[2\]
///     - lv2 pt: 1 == Accessing PPN\[1\]
///     - lv1 pt: 0 == Accessing PPN\[0\]
fn get_pte(pta: usize, va: usize, level: usize) -> &'static mut PageTableEntry {
    assert!(level <= 2);
    let pt = unsafe { &mut *(pta as *mut PageTable) };
    let pte = &mut pt[vpn(va, level)];
    if level == 0 {
        return pte;
    }
    if pte.is(PTE_V) {
        // query from the next level page table
        get_pte(pte.va(), va, level - 1)
    } else {
        // create next level page table
        let new_pt = alloc_page();
        pte.link(new_pt);
        get_pte(new_pt, va, level - 1)
    }
}

fn kvmmap(pta: usize, va: usize, pa: usize, size: usize, flag: usize) {
    let mut addr = page_down(va);
    let end = page_down(va + size);
    let mut pa = page_down(pa);
    loop {
        let pte = get_pte(pta, addr, 2);
        pte.set_pa(pa, flag);
        addr += PGSIZE;
        pa += PGSIZE;
        if addr > end {
            break;
        }
    }
}
