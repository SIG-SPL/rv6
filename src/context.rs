use riscv::register::scause::{self, Exception, Interrupt, Trap};
use riscv::register::stvec;
pub type Reg = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct Context {
    pub regs:       [Reg; 32],
    pub sstatus:    Reg, 
    pub sepc:       Reg,
    pub scause:     Reg,
}

#[no_mangle]
pub fn trap_handler(cx: &mut Context) {
    let scause = scause::read();

    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            println!("SupervisorTimer");
            cx.sepc += 4;
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            panic!("IllegalInstruction")
        }
        _ => panic!("unhandled trap {:?}\n{:#x?}", scause.cause(), cx),
    }
    todo!("Handle trap here!")
}

pub fn trap_init() {
    extern "C" {
        fn __trap();
    }

    unsafe {
        stvec::write(__trap as usize, stvec::TrapMode::Direct);
    }
}
