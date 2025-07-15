#![no_std]
#![no_main]

use bootloader_api::{config::BootloaderConfig, entry_point, info::BootInfo};
use core::panic::PanicInfo;

// Re-route modules located outside `src`
#[path = "../memory/mod.rs"]
pub mod memory;
#[path = "../scheduler/mod.rs"]
pub mod scheduler;
#[path = "../telemetry/mod.rs"]
pub mod telemetry;
#[path = "../arch/mod.rs"]
pub mod arch;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        // AIKLE: placeholder for serial/VGA output
    }};
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
    scheduler::spawn(|| {
        loop {
            println!("hello from task 0!");
            scheduler::yield_now();
        }
    });

    scheduler::run()
}
