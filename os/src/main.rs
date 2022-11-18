// os/src/main.rs

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
//上面不需要加pub的原因是同级
//下面需要加pub的原因是子级
pub mod sync;
pub mod syscall;
pub mod trap;
pub mod task;

use core::arch::global_asm;
#[cfg(feature = "board_qemu")]
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

