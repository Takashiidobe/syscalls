use std::arch::asm;

#[cfg(any(not(target_os = "linux")))]
compile_error!("Only works on linux");

macro_rules! syscall0 {
    ($name:ident, $nr:expr) => {
        extern "C" fn $name() {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                asm!(
                    "mov x0, #0",
                    "svc #0",
                    in("w8") $nr
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                    in("rsi") arg2.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                    in("rsi") arg2.into(),
                    in("rdx") arg3.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                    in("rsi") arg2.into(),
                    in("rdx") arg3.into(),
                    in("r10") arg4.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                    in("x4") arg5.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                    in("rsi") arg2.into(),
                    in("rdx") arg3.into(),
                    in("r10") arg4.into(),
                    in("r9") arg5.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
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
                    in("w8") $nr,
                    in("x0") arg1.into(),
                    in("x1") arg2.into(),
                    in("x2") arg3.into(),
                    in("x3") arg4.into(),
                    in("x4") arg5.into(),
                    in("x5") arg6.into(),
                );
                #[cfg(target_arch = "x86_64")]
                asm!(
                    "syscall",
                    in("rax") $nr,
                    in("rdi") arg1.into(),
                    in("rsi") arg2.into(),
                    in("rdx") arg3.into(),
                    in("r10") arg4.into(),
                    in("r9") arg5.into(),
                    in("r8") arg6.into(),
                );
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                compile_error!("not implemented");
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
syscall0!(exit, 93);
#[cfg(target_arch = "aarch64")]
syscall3!(write, 64);

#[cfg(target_arch = "x86_64")]
syscall0!(exit, 60);
#[cfg(target_arch = "x86_64")]
syscall3!(write, 1);

fn main() {
    #[cfg(target_arch = "aarch64")]
    let string = "Hello ARM64\n";
    #[cfg(target_arch = "x86_64")]
    let string = "Hello x86\n";
    let ptr = string.as_ptr() as usize;
    let len = string.len();
    write(1usize, ptr, len);
    exit();
}
