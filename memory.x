MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 16K
    RAM : ORIGIN = 0x20000000, LENGTH = 2K
}

ENTRY(_start)

SECTIONS
{
    .text : {
        KEEP(*(.init));
        *(.text .text.*);
        *(.rodata .rodata.*);
    } > FLASH

    .data : {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM AT > FLASH

    _sidata = LOADADDR(.data);

    .bss : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM

    /DISCARD/ : {
        *(.eh_frame);
    }
}