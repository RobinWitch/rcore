// os/src/main.rs
// 关于mod，有意思的一点是只有在mod.rs或者main.rs文件夹下被声明，其他的文件都只用use
#![no_std]
//告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core（core库不需要操作系统的支持）
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod loader;
mod config;
mod timer;
//上面不需要加pub的原因是同级
//下面需要加pub的原因是子级
pub mod sync;
pub mod syscall;
pub mod trap;
pub mod task;

use core::arch::global_asm;
#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Je t'aime,Je t'aime,Mon Amour");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt(); //设置了 sie.stie 使得 S 特权级时钟中断不会被屏蔽
    timer::set_next_trigger();      //设置第一个 10ms 的计时器
    task::run_first_task();
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

