#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;

#[entry]
fn main() -> ! {
    let mut uart = unsafe { pl011_uart::Pl011Uart::from_address(0x0900_0000) }.unwrap();
    uart.write_bytes(b"Hello, world!\r\n");

    {{crate_name}}::qemu_exit(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    {{crate_name}}::qemu_exit(1)
}
