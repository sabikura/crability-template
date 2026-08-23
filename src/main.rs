#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;
{%- if platform == "qemu-el1" %}
use {{crate_name}}::uprintln;
{%- else %}
use {{crate_name}}::{el2_entry, uprintln};

el2_entry!();
{%- endif %}

#[entry]
fn main() -> ! {
    uprintln!("hello, world!");

{%- if platform != "fvp" %}
    {{crate_name}}::qemu::exit(0)
{%- else %}
    loop {}
{%- endif %}
}
