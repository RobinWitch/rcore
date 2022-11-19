#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;


/// 使用no_mangle标记这个函数，来对它禁用名称重整（name mangling）——这确保Rust编译器输出一个名为_start的函数；
/// 否则，编译器可能最终生成名为_ZN3blog_os4_start7hb173fedf945531caE的函数，无法让链接器正确辨别。
/// 就是这个位置de我几个小时,
/// 就是因为#[no_mangle]没加
#[no_mangle] 
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

/// 我们使用 Rust 的宏将其函数符号 main 标志为弱链接。这样在最后链接的时候，虽然在 lib.rs 和 bin 目录下的某个应用程序都有 main 符号，但
/// 由于 lib.rs 中的 main 符号是弱链接，链接器会使用 bin 目录下的应用主逻辑作为 main 。
/// 这里我们主要是进行某种程度上的保护，如果在 bin 目录下找不到任何 main ，那么编译也能够通过，但会在运行时报错。
/// 为了支持上述这些链接操作，我们需要在 lib.rs 的开头加入：#![feature(linkage)]
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}





use syscall::*;
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }
pub fn yield_() -> isize { sys_yield() }
pub fn get_time() -> isize { sys_get_time() }