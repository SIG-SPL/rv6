/// Write a value to a general purpose register.
#[macro_export]
macro_rules! write_reg {
    ($name:tt, $value:expr) => {
        unsafe {
            core::arch::asm!(concat!("mv ", stringify!($name), ", {}"), in(reg) $value);
        }
    };
}

/// Read a value from a general purpose register.
#[macro_export]
macro_rules! read_reg {
    ($name:tt) => {
        {
            let value: usize;
            unsafe {
                core::arch::asm!(concat!("mv {}, ", stringify!($name)), out(reg) value);
            }
            value
        }
    };
}

#[macro_export]
macro_rules! load_address {
    ($name:tt, $fn:tt) => {
        unsafe {
            core::arch::asm!(concat!("la ", stringify!($name), ", ", stringify!($fn)));
        }
    };
}

#[macro_export]
macro_rules! call {
    ($fn:tt) => {
        unsafe {
            core::arch::asm!(concat!("call ", stringify!($fn)));
        }
    };
}

#[macro_export]
macro_rules! sret {
    () => {
        unsafe {
            core::arch::asm!("sret");
        }
    };
}

#[macro_export]
macro_rules! ret {
    () => {
        unsafe {
            core::arch::asm!("ret");
        }
    };
}
