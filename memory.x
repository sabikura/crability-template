{% if platform == "qemu" -%}
/* RAM on QEMU's virt machine starts at 0x40000000 */
MEMORY {
    el2 : ORIGIN = 0x40000000, LENGTH = 2K
    ram : ORIGIN = 0x40000800, LENGTH = 64M - 2K
}

PROVIDE(__el1_stack_size = 0x4000);
/* QEMU jumps to the ELF entry point at EL2, which must be the EL2 stub blob */
ENTRY(EL2_ENTRY_BIN)
{% elsif platform == "qemu-el1" -%}
/* RAM on QEMU's virt machine starts at 0x40000000 */
MEMORY {
    ram : ORIGIN = 0x40000000, LENGTH = 64M
}

PROVIDE(__el1_stack_size = 0x4000);
/* Plain virt jumps to the ELF entry point at EL1 in A64. The runtime's hybrid
   feature makes _el1_entry start with the A64 prologue that enables capabilities
   and switches to C64, so it can be entered directly */
ENTRY(_el1_entry)
{% else -%}
/* On the FVP, Trusted Firmware loads BL33 at 0xE0000000 */
MEMORY {
    el2 : ORIGIN = 0xE0000000, LENGTH = 2K
    ram : ORIGIN = 0xE0000800, LENGTH = 64M - 2K
}

PROVIDE(__el1_stack_size = 0x4000);
ENTRY(_el1_entry)
{% endif -%}
{% if platform != "qemu-el1" %}
SECTIONS {
    .el2_entry : {
        KEEP(*(.el2_entry))
        . = ALIGN(0x800);
    } > el2

    /DISCARD/ : { *(.comment) *(.note.*) *(.eh_frame) }
}
{%- else %}
SECTIONS {
    /DISCARD/ : { *(.comment) *(.note.*) *(.eh_frame) }
}
{%- endif %}
