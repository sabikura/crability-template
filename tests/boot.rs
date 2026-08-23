#![no_std]
#![no_main]

//! Integration test booted on QEMU by `cargo test`. The custom assert macros make
//! QEMU exit non-zero on failure; reaching the end exits zero.

use aarch64_purecap_rt::entry;
{%- if platform == "qemu-el1" %}
use {{crate_name}}::{test_assert, test_assert_eq, uprintln};
{%- else %}
use {{crate_name}}::{el2_entry, test_assert, test_assert_eq, uprintln};

el2_entry!();
{%- endif %}

#[entry]
fn main() -> ! {
    uprintln!("running boot tests");

    stack_readback();
    capability_roundtrip();

    uprintln!("boot tests passed");
    {{crate_name}}::qemu::exit(0)
}

/// Stores and loads through a purecap stack pointer
fn stack_readback() {
    let mut values = [0u64; 4];
    for (i, value) in values.iter_mut().enumerate() {
        *value = (i as u64) * 3;
    }

    test_assert_eq!(values[3], 9);
    test_assert!(
        values.iter().sum::<u64>() == 18,
        "sum was ",
        values.iter().sum::<u64>()
    );
}

/// A pointer derived from an array keeps working through a round-trip
fn capability_roundtrip() {
    let data = [0xAAu8, 0xBB, 0xCC];
    let ptr = data.as_ptr();

    // SAFETY: ptr points at data, which outlives the reads
    let back = unsafe { core::slice::from_raw_parts(ptr, data.len()) };
    test_assert_eq!(back[2], 0xCC);
}
