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

impl TrapFrame {
    pub fn new() -> Self {
        Self {
            regs: [0; 32],
            sstatus: 0,
            sepc: 0,
            scause: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

impl Context {
    pub fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }
}
