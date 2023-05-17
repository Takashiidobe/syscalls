use std::arch::asm;

macro_rules! syscall0 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name() {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    };
}

macro_rules! syscall1 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall2 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>, arg2: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall3 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>, arg2: impl Into<usize>, arg3: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall4 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>, arg2: impl Into<usize>, arg3: impl Into<usize>, arg4: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall5 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>, arg2: impl Into<usize>, arg3: impl Into<usize>, arg4: impl Into<usize>, arg5: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                    in("x4") arg5.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

macro_rules! syscall6 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name(arg1: impl Into<usize>, arg2: impl Into<usize>, arg3: impl Into<usize>, arg4: impl Into<usize>, arg5: impl Into<usize>, arg6: impl Into<usize>) {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr as usize,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                    in("x4") arg5.into(),
                    in("x5") arg6.into(),
                );
                #[cfg(not(target_arch = "aarch64"))]
                compile_error!("not implemented");
            }
        }
    }
}

syscall0!(exit, 93);
syscall3!(write, 64);

fn main() {
    let string = "Hello ARM64\n";
    let ptr = string.as_ptr();
    let len = string.len();
    write(1usize, ptr, len);
    exit();
}
