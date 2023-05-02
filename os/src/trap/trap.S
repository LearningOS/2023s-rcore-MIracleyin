.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2 # 4 字节对齐
__alltraps:
    csrrw sp, sscratch, sp # sscratch -> sp, sp -> sscratch, 交换 sp 与 sscratch，在这一行执行之前 sp 指向用户栈，sscratch 指向内核栈；sscratch 保存内核栈地址，可以作为中转站让 sp 暂存在 sscratch
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8 # 在内核栈上分配 -34*8
    # save general-purpose registers x0 总是 0 不需要保存
    sd x1, 1*8(sp) # x1 -> 1*8(sp)
    # skip sp(x2), we will save it later，目前用于 sp 保存寄存器内容
    sd x3, 3*8(sp) 
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus # sstatus -> t0
    csrr t1, sepc # sepc -> t1
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch # sscratch 保存了最开始 sp 的地址，现在写回 t2 sscratch(sp) -> t2
    sd t2, 2*8(sp) # t2 写入内核栈
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp # sp -> a0 指向 保存在内核栈上的上下文 
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0 # a0 -> sp 暂时忽略，sp仍然指向内核栈栈顶
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc 首先恢复 CSR
    ld t0, 32*8(sp) # 32*8(sp) -> t0
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0 # t0 -> sstatus
    csrw sepc, t1 # t1 -> sepc
    csrw sscratch, t2 # t2-> sscratch
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp) 
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8 # 释放内核栈空间
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp # sscratch -> sp sp -> sscratch sp 指向用户栈顶 sscratch 指回内核栈顶
    sret # 指回 U 特权级继续运行应用程序控制流
