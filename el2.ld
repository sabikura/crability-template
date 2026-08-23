{% if platform == "qemu" -%}
MEMORY {
    ram : ORIGIN = 0x40000000, LENGTH = 2K
}

PROVIDE(__el1_entry_start = 0x40000800);
{% else -%}
MEMORY {
    ram : ORIGIN = 0xE0000000, LENGTH = 2K
}

PROVIDE(__el1_entry_start = 0xE0000800);
{% endif -%}
ENTRY(_el2_drop_to_el1);

SECTIONS {
    .text : {
        KEEP(*(.text._el2_drop_to_el1))
    } > ram

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
