use crate::println;
use bootloader_api::info::BootInfo;

pub fn init(_boot_info: &mut BootInfo) {
    // Init GDT, IDT, PIC, etc.
    println!("[x86_64] arch init");
    // Report simple CPU information
    unsafe {
        let id = core::arch::x86_64::__cpuid(0);
        let bytes = [
            id.ebx as u8,
            (id.ebx >> 8) as u8,
            (id.ebx >> 16) as u8,
            (id.ebx >> 24) as u8,
            id.edx as u8,
            (id.edx >> 8) as u8,
            (id.edx >> 16) as u8,
            (id.edx >> 24) as u8,
            id.ecx as u8,
            (id.ecx >> 8) as u8,
            (id.ecx >> 16) as u8,
            (id.ecx >> 24) as u8,
        ];
        let vendor = core::str::from_utf8_unchecked(&bytes);
        println!("CPU vendor: {}", vendor);
    }
}
