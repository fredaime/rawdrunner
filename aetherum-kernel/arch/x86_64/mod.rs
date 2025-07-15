use bootloader_api::info::BootInfo;
use crate::println;

pub fn init(_boot_info: &mut BootInfo) {
    // Init GDT, IDT, PIC, etc.
    println!("[x86_64] arch init");
    // AIKLE: placeholder — feed CPU topology to AI
}
