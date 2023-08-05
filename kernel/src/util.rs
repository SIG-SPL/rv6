/// Write a value to a general purpose register.
#[macro_export]
macro_rules! write_reg {
    ($name:tt, $value:expr) => {
        #[allow(unused_unsafe)]
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
            #[allow(unused_unsafe)]
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

#[macro_export]
/// The value of cpuid is now always 0.
/// Should be implemented in the future if we want to support multi-core.
macro_rules! cpuid {
    () => {
        0
    };
}
