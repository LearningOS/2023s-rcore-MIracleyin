//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct); // stvec 设置为 Direct，并指向 trap.S 的函数 __alltraps 将 Trap 上下文保存在内核栈上
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext { // 原样返回 TrapContext
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() { // 根据 scause 寄存器的值所保存的 Trap 原因进行分发处理
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // 首先修改 spec 
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize; // 从 Trap 上下文中取出三个参数 a0~a2 传给 syscall 函数获取返回值
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => { // 访存错误 非法指令错误
            println!("[kernel] PageFault in application, kernel killed it."); 
            run_next_app(); // 错误，切换运行下一个应用程序
        }
        Trap::Exception(Exception::IllegalInstruction) => { // 
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app(); // 错误，切换运行下一个应用程序
        }
        _ => { // 其他不支持的 Trap 类型，panic
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
