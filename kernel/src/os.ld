OUTPUT_ARCH(riscv)
ENTRY(_start)

PHY_START    = 0x80000000;
PHY_SIZE     = 128M;
OPENSBI_SIZE = 0x00200000;
VM_START     = 0xffffffe000000000;
VM_END       = 0xffffffff00000000;
VM_SIZE      = VM_END - VM_START;

MEMORY {
    ram  (wxa): ORIGIN = PHY_START + OPENSBI_SIZE, LENGTH = PHY_SIZE - OPENSBI_SIZE
    ramv (wxa): ORIGIN = VM_START  + OPENSBI_SIZE, LENGTH = VM_SIZE  - OPENSBI_SIZE
}

BASE_ADDRESS = VM_START + OPENSBI_SIZE;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    } >ramv AT>ram

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } >ramv AT>ram

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    } >ramv AT>ram

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    } >ramv AT>ram

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
