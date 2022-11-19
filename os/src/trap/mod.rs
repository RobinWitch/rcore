// os/src/trap/mod.rs


/// 
/// Trap 处理的总体流程如下：首先通过 __alltraps 将 Trap 上下文保存在内核栈上，
/// 然后跳转到使用 Rust 编写的 trap_handler 函数完成 Trap 分发及处理。
/// 当 trap_handler 返回之后，使用 __restore 从保存在内核栈上的 Trap 上下文恢复寄存器。
/// 最后通过一条 sret 指令回到应用程序执行。
/// 
/// 进入 S 特权级 Trap 的相关 CSR(Control and Status Register)
/// sstatus SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
/// sepc    当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
/// scause  描述 Trap 的原因
/// stval   给出 Trap 附加信息
/// stvec   控制 Trap 处理代码的入口地址


pub mod context;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap,Interrupt},
    stval, stvec,sie,
};
pub use context::TrapContext;
use crate::timer::set_next_trigger;
use crate::task::suspend_current_and_run_next;




global_asm!(include_str!("trap.S"));
/// 修改 stvec 寄存器来指向正确的 Trap 处理入口点
pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;   // aim to point at the next instrument
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            panic!("[kernel] Cannot continue!");
            //run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
            //run_next_app();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}