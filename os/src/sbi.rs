// os/src/sbi.rs
// some sbi functions

fn sbi_call(extension: usize, function: usize, args: &[usize; 3]) -> isize {
    let mut error_code;
    unsafe {
        core::arch::asm!("ecall",
            // error code in a0, ret in a1
            inlateout("x10") args[0] => error_code,
            inlateout("x11") args[1] => _,
            in("x12") args[2],
            in("x17") extension,
            in("x16") function,
        )
    }
    error_code
}

const SYSTEM_RESET_EXTENSION: usize = 0x53525354;
const SYSTEM_RESET_FUNCTION: usize = 0;

pub fn shutdown() -> ! {
    sbi_call(SYSTEM_RESET_EXTENSION, SYSTEM_RESET_FUNCTION, &[0; 3]);
    panic!("Shutdown expected!");
}
