#![no_std]
#![no_main]

use bootloader_api::{config::BootloaderConfig, entry_point, info::BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// Re-route modules located outside `src`
#[path = "../arch/mod.rs"]
pub mod arch;
#[path = "../memory/mod.rs"]
pub mod memory;
#[path = "../scheduler/mod.rs"]
pub mod scheduler;
#[path = "../telemetry/mod.rs"]
pub mod telemetry;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        $crate::serial::_print(format_args!($($arg)*));
    }};
}

mod serial {
    use super::*;
    lazy_static! {
        static ref SERIAL1: Mutex<SerialPort> = {
            let mut sp = unsafe { SerialPort::new(0x3F8) };
            sp.init();
            Mutex::new(sp)
        };
    }

    pub fn _print(args: core::fmt::Arguments) {
        use core::fmt::Write;
        SERIAL1.lock().write_fmt(args).ok();
        SERIAL1.lock().write_str("\n").ok();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {info}");
    loop {}
}

// Configure bootloader (higher-half, paging, etc.)
const BOOT_CFG: BootloaderConfig = {
    let mut cfg = BootloaderConfig::new_default();
    cfg.kernel_stack_size = 8 * 4096;
    cfg
};

// Boot entry-point
entry_point!(kernel_main, config = &BOOT_CFG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    arch::init(boot_info);
    memory::init(boot_info);
    telemetry::init();
    scheduler::init();

    println!("[Aetherum] boot OK — entering idle loop");

    // Spawn an example task
    scheduler::spawn(|| loop {
        println!("hello from task 0!");
        scheduler::yield_now();
    });

    scheduler::run()
}
