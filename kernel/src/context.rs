pub type Reg = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct TrapFrame {
    pub regs:       [Reg; 32],
    pub sstatus:    Reg, 
    pub sepc:       Reg,
    pub scause:     Reg,
}

/// Context for task switching
pub struct Context {
    pub ra: Reg,
    pub sp: Reg,

    // Callee-saved registers
    pub s0: Reg,
    pub s1: Reg,
    pub s2: Reg,
    pub s3: Reg,
    pub s4: Reg,
    pub s5: Reg,
    pub s6: Reg,
    pub s7: Reg,
    pub s8: Reg,
    pub s9: Reg,
    pub s10: Reg,
    pub s11: Reg,
}
