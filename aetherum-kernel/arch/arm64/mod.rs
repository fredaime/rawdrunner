// Stub for future arm64 bring-up
#![allow(dead_code)]
use crate::println;

pub fn init(_: &mut bootloader_api::info::BootInfo) {
    // Basic stub for arm64 initialization
    println!("[arm64] arch init (stub)");
}
