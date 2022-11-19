// os/src/lang_item.rs

use crate::syscall::shutdown;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else { 
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}

