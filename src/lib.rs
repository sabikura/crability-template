#![no_std]

//! Support code shared by the application and the integration tests: UART printing
{%- if platform == "qemu-el1" %}
//! and the panic handler. Booting needs no glue here: the runtime's hybrid feature
//! makes `_el1_entry` directly enterable at EL1 in A64.
{%- else %}
//! , the EL2 entry blob embedding and the panic handler.
{%- endif %}
//!
//! Printing avoids `core::fmt` on purpose: the current purecap toolchain copies
//! `fmt::Arguments` with NEON registers, which strips capability tags and faults on
//! the first formatted print. The [`uart::Print`] trait fills that gap.

{%- if platform != "qemu-el1" %}
/// Embed the EL2 entry blob assembled by build.rs. Every binary of this package
/// (the application and each integration test) must invoke this once at the top level
/// so the blob lands in its `.el2_entry` section.
#[macro_export]
macro_rules! el2_entry {
    () => {
        #[link_section = ".el2_entry"]
        #[used]
        #[no_mangle]
        pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
            *include_bytes!(env!("EL2_ENTRY_BIN"));
    };
}
{%- endif %}

pub mod uart {
{%- if platform != "fvp" %}
    // QEMU's virt machine puts the PL011 at 0x0900_0000
    const UART_PL011_DATA_REGISTER: usize = 0x0900_0000;
{%- else %}
    // The Morello FVP's AP UART (PL011)
    const UART_PL011_DATA_REGISTER: usize = 0x2A40_0000;
{%- endif %}

    pub fn write_byte(byte: u8) {
        let uart_dr = aarch64_purecap_cpu::capability_from_address(UART_PL011_DATA_REGISTER);
        // SAFETY: The ptr to the UART PL011 data register is valid for writes and properly
        // aligned
        unsafe {
            core::ptr::write_volatile(uart_dr, byte as u32);
        }
    }

    pub fn write_str(s: &str) {
        for byte in s.bytes() {
            write_byte(byte);
        }
    }

    /// Formatting-free printing for the `uprint!`/`uprintln!` macros. Implement it for
    /// your own types to use them in `test_assert_eq!`.
    pub trait Print {
        fn print(&self);
    }

    impl Print for &str {
        fn print(&self) {
            write_str(self);
        }
    }

    impl Print for bool {
        fn print(&self) {
            write_str(if *self { "true" } else { "false" });
        }
    }

    impl Print for char {
        fn print(&self) {
            let mut buf = [0u8; 4];
            write_str(self.encode_utf8(&mut buf));
        }
    }

    /// Prints as hex instead of the decimal integer impls, e.g. `uprintln!(Hex(addr))`
    pub struct Hex(pub u64);

    impl Print for Hex {
        fn print(&self) {
            write_str("0x");
            let digits = (self.0.leading_zeros() as usize / 4).min(15);
            for i in digits..16 {
                let nibble = (self.0 >> ((15 - i) * 4)) & 0xF;
                write_byte(b"0123456789abcdef"[nibble as usize]);
            }
        }
    }

    fn write_u64(mut value: u64) {
        let mut buf = [0u8; 20];
        let mut at = buf.len();
        loop {
            at -= 1;
            buf[at] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        for &byte in &buf[at..] {
            write_byte(byte);
        }
    }

    fn write_i64(value: i64) {
        if value < 0 {
            write_byte(b'-');
        }
        write_u64(value.unsigned_abs());
    }

    macro_rules! print_as_u64 {
        ($($ty:ty),*) => {
            $(impl Print for $ty {
                fn print(&self) {
                    write_u64(*self as u64);
                }
            })*
        };
    }

    macro_rules! print_as_i64 {
        ($($ty:ty),*) => {
            $(impl Print for $ty {
                fn print(&self) {
                    write_i64(*self as i64);
                }
            })*
        };
    }

    print_as_u64!(u8, u16, u32, u64, usize);
    print_as_i64!(i8, i16, i32, i64, isize);
}

{% raw -%}
/// Print a comma-separated list of [`uart::Print`] values, e.g.
/// `uprint!("x is ", x)`. Not a `print!`: there is no format string.
#[macro_export]
macro_rules! uprint {
    ($($arg:expr),* $(,)?) => {{
        $( $crate::uart::Print::print(&$arg); )*
    }};
}

/// Like `uprint!`, with a trailing CRLF
#[macro_export]
macro_rules! uprintln {
    ($($arg:expr),* $(,)?) => {{
        $( $crate::uart::Print::print(&$arg); )*
        $crate::uart::write_str("\r\n");
    }};
}
{%- endraw %}
{% if platform != "fvp" %}
pub mod qemu {
    /// Terminate QEMU itself with the given exit code, through a semihosting
    /// SYS_EXIT call. Requires QEMU to run with `-semihosting` (run-qemu.sh does).
    pub fn exit(code: u64) -> ! {
        const SYS_EXIT: u64 = 0x18;
        const ADP_STOPPED_APPLICATION_EXIT: u64 = 0x20026;

        let block = [ADP_STOPPED_APPLICATION_EXIT, code];
        loop {
            // SAFETY: Semihosting call, only reads the block behind x1
            unsafe {
                core::arch::asm!(
                    "hlt #0xf000",
                    in("x0") SYS_EXIT,
                    in("x1") block.as_ptr() as usize,
                );
            }
        }
    }
}

{% raw -%}
/// Like `assert!`, but reports over the UART and makes QEMU exit non-zero, so
/// `cargo test` sees the failure. The optional extra arguments are `uart::Print`
/// values, not a format string.
#[macro_export]
macro_rules! test_assert {
    ($cond:expr $(, $($arg:expr),+ $(,)?)?) => {
        if !$cond {
            $crate::uprintln!(
                "FAILED: ", stringify!($cond), " at ", file!(), ":", line!()
            );
            $( $crate::uprintln!("  ", $($arg),+); )?
            $crate::qemu::exit(1);
        }
    };
}

/// Like `assert_eq!` for `uart::Print` values, reporting over the UART and making
/// QEMU exit non-zero
#[macro_export]
macro_rules! test_assert_eq {
    ($left:expr, $right:expr $(, $($arg:expr),+ $(,)?)?) => {{
        let left = $left;
        let right = $right;
        if left != right {
            $crate::uprintln!(
                "FAILED: ", stringify!($left), " == ", stringify!($right),
                " at ", file!(), ":", line!()
            );
            $crate::uprintln!("  left:  ", left);
            $crate::uprintln!("  right: ", right);
            $( $crate::uprintln!("  ", $($arg),+); )?
            $crate::qemu::exit(1);
        }
    }};
}
{%- endraw %}
{% endif %}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // The panic message is fmt-based, so only the location is printed
    uart::write_str("panic");
    if let Some(location) = info.location() {
        crate::uprint!(" at ", location.file(), ":", location.line());
    }
    uart::write_str("\r\n");

{%- if platform != "fvp" %}
    qemu::exit(1)
{%- else %}
    loop {}
{%- endif %}
}
