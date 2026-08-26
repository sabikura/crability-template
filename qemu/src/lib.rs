#![no_std]
#![feature(strict_provenance)]

/// Terminate QEMU with the given exit code using semihosting
pub fn qemu_exit(code: u64) -> ! {
    const SYS_EXIT: u64 = 0x18;
    const ADP_STOPPED_APPLICATION_EXIT: u64 = 0x20026;

    let block = [ADP_STOPPED_APPLICATION_EXIT, code];
    loop {
        // SAFETY: Semihosting call
        unsafe {
            core::arch::asm!(
                "hlt #0xf000",
                in("x0") SYS_EXIT,
                in("x1") block.as_ptr() as usize,
            );
        }
    }
}
