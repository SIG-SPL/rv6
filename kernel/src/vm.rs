//! Sv39 Mode Virtual Memory Management

use config::{layout::*, vm::*};

static mut INIT_PT: PageTable = PageTable::new();

#[no_mangle]
#[link_section = ".text.entry"]
pub fn boot_mmap() {
    // use riscv::register::satp;

    kvmmap(
        unsafe { &mut INIT_PT as *mut PageTable as usize },
        VM_START,
        PHY_START,
        PHY_SIZE,
        2,
        Privileges::Read as usize | Privileges::Write as usize | Privileges::Execute as usize,
    );
    // write_reg!(sp, read_reg!(sp) + PA2VA_OFFSET);
    // write_reg!(ra, read_reg!(ra) + PA2VA_OFFSET);
    // unsafe {
    //     satp::set(
    //         satp::Mode::Sv39,
    //         0,
    //         &INIT_PT as *const PageTable as usize >> PGSHIFT,
    //     );
    //     riscv::asm::sfence_vma(0, 0);
    // }
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

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageTableEntry")
            .field("bits", &self.bits)
            .finish()
    }
}

impl PageTableEntry {
    const fn default() -> Self {
        Self { bits: 0 }
    }

    /// Link the PTE to the next level page table
    /// - next: The virtual address of the next level page table
    /// - flag: The privileges of the PTE
    fn link(&mut self, next: usize, flag: usize) {
        let ppn = (next - PA2VA_OFFSET) >> PGSHIFT;
        self.bits = (ppn << PTE_SHIFT) | flag;
    }

    fn set_pa(&mut self, pa: usize, flag: usize) {
        let ppn = pa >> PGSHIFT;
        self.bits = (ppn << PTE_SHIFT) | Privileges::Vaild as usize | flag;
    }

    #[inline(always)]
    pub fn is(&self, flag: Privileges) -> bool {
        self.bits & (flag as usize) != 0
    }

    /// Extract the physical page number from a PTE.
    #[inline(always)]
    pub fn ppn(&self) -> usize {
        self.bits >> PTE_SHIFT
    }

    #[inline(always)]
    pub fn va(&self) -> usize {
        (self.ppn() << PGSHIFT) + PA2VA_OFFSET
    }
}

/// Get the page table entry for a virtual address.
/// - pt: Root Page table
/// - va: Virtual address
/// - level: The level of the lowest level page table
///     - 3-lvl pt: 0
///     - 2-lvl pt: 1
///     - 1-lvl pt: 2
fn get_pte(pta: usize, va: usize, level: usize) -> usize {
    assert!(level <= 2);
    let pt = unsafe { &mut *(pta as *mut PageTable) };
    let pte = &mut pt[vpn(va, level)];
    if level == 0 {
        return pte as *mut PageTableEntry as usize;
    }
    if pte.is(Privileges::Vaild) {
        // query from the next level page table
        get_pte(pte.va(), va, level - 1)
    } else {
        // create next level page table
        let new_pt = PageTable::new();
        pte.link(
            &new_pt as *const PageTable as usize,
            Privileges::Vaild as usize,
        );
        get_pte(pta, va, level - 1)
    }
}

fn kvmmap(pta: usize, va: usize, pa: usize, size: usize, level: usize, flag: usize) {
    let mut addr = page_down(va, level);
    let end = page_down(va + size - 1, level);
    let mut pa = page_down(pa, level);
    loop {
        let pte = unsafe { &mut *(get_pte(pta, addr, level) as *mut PageTableEntry) };
        pte.set_pa(pa, flag);
        addr += pagesize(level);
        pa += pagesize(level);
        if addr > end {
            break;
        }
    }
}
