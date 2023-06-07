.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __trap
    .globl __restore
    .align 2
__trap:
    # csrrw sp, sscratch, sp
    # now sp->kernel stack, sscratch->user stack
    addi sp, sp, -(32+3)*8
    # save general-purpose registers
    sd x1, 1*8(sp)
    # save x3~x31
    .set n, 3
    .rept 29
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    csrr t2, scause
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    sd t2, 34*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    # csrr t2, sscratch
    mv a0, sp
    call trap_handler

__restore:
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 34*8(sp)
    csrw sstatus, t0
    csrw sepc,    t1
    csrw scause,  t2
    ld x1, 1*8(sp)
    .set n, 3
    .rept 29
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, (32+3)*8
    # now sp->kernel stack, sscratch->user stack
    # csrrw sp, sscratch, sp
    sret
