// os/src/timer.rs

use riscv::register::time;
use crate::sbi::set_timer;

pub fn get_time() -> usize {
    time::read()
}

use crate::config::CLOCK_FREQ;
const TICKS_PER_SEC: usize = 100;

//timer 子模块的 set_next_trigger 函数对 set_timer 进行了封装，它首先读取当前 mtime 的值，然后计算出 10ms 之内计数器的增量，再将 mtimecmp 设置为二者的和。这样，10ms 之后一个 S 特权级时钟中断就会被触发
//常数 CLOCK_FREQ 是一个预先获取到的各平台不同的时钟频率，单位为赫兹，也就是一秒钟之内计数器的增量。它可以在 config 子模块中找到。CLOCK_FREQ 除以常数 TICKS_PER_SEC 即是下一次时钟中断的计数器增量值
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

const MICRO_PER_SEC: usize = 1_000_000;
// timer 子模块的 get_time_us 以微秒为单位返回当前计数器的值
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}


const MSEC_PER_SEC: usize = 1000;
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
