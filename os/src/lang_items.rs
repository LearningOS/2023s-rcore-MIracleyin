//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}", // 如果可以定位问题，那么给出位置、行号和原因
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap()); // 如果定位不到，那么直接传出原因
    }
    shutdown() // 关机
}
