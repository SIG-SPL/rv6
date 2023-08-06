use config::{plic::*, layout::*};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum ExternalInterrupt {
    None,
    VirtIO(u32),
    UART,
    RTC,
    // PCI-E & Virt Platform Bus not supported yet
}

impl From<u32> for ExternalInterrupt {
    fn from(value: u32) -> Self {
        match value {
            1..=8 => ExternalInterrupt::VirtIO(value - 1),
            10 => ExternalInterrupt::UART,
            11 => ExternalInterrupt::RTC,
            _ => ExternalInterrupt::None,
        }
    }
}

impl ExternalInterrupt {
    fn as_u32(&self) -> u32 {
        match self {
            ExternalInterrupt::None => 0,
            ExternalInterrupt::VirtIO(v) => *v + 1,
            ExternalInterrupt::UART => 10,
            ExternalInterrupt::RTC => 11,
        }
    }

    /// Set the global interrupt source priority
    fn set_priority(&self, priority: u8) -> &Self {
        unsafe {
            plic_pri(self.as_u32() as usize).write_volatile(priority as u32 & 7);
        }
        self
    }

    /// Enable the interrupt source for a specific CPU.
    fn enable(&self, hartid: usize) -> &Self {
        let enables = plic_sen(hartid);
        unsafe {
            enables.write_volatile(enables.read_volatile() | 1 << self.as_u32());
        }
        self
    }

    /// Complete the pending interrupt for the current CPU.
    pub fn complete(&self) {
        unsafe {
            plic_sclaim(cpuid!()).write_volatile(self.as_u32());
        }
    }

}

pub fn init(hartid: usize) {
    ExternalInterrupt::UART.enable(hartid).set_priority(7);
    ExternalInterrupt::VirtIO(0).enable(hartid).set_priority(7);
    set_threshold(hartid, 0);
}

/// Set the global PLIC priority threshold.
/// The PLIC will mask any interrupts at or below the given threshold.
fn set_threshold(hartid: usize, thrs: u8) {
    unsafe {
        plic_spri(hartid).write_volatile(thrs as u32 & 7)
    }
}

/// Get the next pending external interrupt for the current CPU.
pub fn next() -> ExternalInterrupt {
    ExternalInterrupt::from(unsafe { plic_sclaim(cpuid!()).read_volatile() })
}