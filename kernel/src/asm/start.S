    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call relocate
    call os_main

# Before call to relocate, we run at physical address
# After the call, we run at virtual address
relocate:
    li t0, 0xFFFFFFDF80000000   # PA2VA_OFFSET
    add ra, ra, t0
    add sp, sp, t0
    la t0, boot_pt
    srli t0, t0, 12             # PPN
    li t1, 8 << 60              # Sv39
    or t0, t0, t1
    csrw satp, t0
    sfence.vma
    ret

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top

boot_stack_top:

    .section .data
    .align 12
    .globl boot_pt, remap_root_pt
boot_pt:
    .quad 0xF                      # map UART & MMIO (equal, RWXV)
    .zero 8
    .quad (0x80000 << 10) | 0xF    # map kernel (equal, RWXV)
    .zero 8 * 381
    .quad (0x80000 << 10) | 0xF    # map kernel (linear, RWXV)
    .zero 8 * 127
