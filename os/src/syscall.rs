// os/src/syscall.rs

// syscall helper
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        // a brief intro to asm!
        // asm (print!-like syntax) first. Here it is "ecall". You can also write several instructions.
        // registers' IOs follow, here are args[0,1,2] -> a0, a1, a2; syscall num -> a7 (Linux riscv ABI)
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        )
    }
    ret
}

const SYSCALL_EXIT: usize = 93;
pub fn sys_exit(xstate: i32) -> ! {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0]);
    panic!("sys_exit unexpectedly returned");
}

const SYSCALL_WRITE: usize = 64;
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
