#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;

#[link_section = ".el2_entry"]
#[used]
#[no_mangle]
pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
    *include_bytes!(env!("EL2_ENTRY_BIN"));

#[entry]
fn main() -> ! {
    let mut uart = unsafe { pl011_uart::Pl011Uart::from_address(0x2A40_0000) }.unwrap();
    uart.write_bytes(b"Hello, world!\r\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
