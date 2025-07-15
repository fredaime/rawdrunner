#![allow(dead_code)]
use core::ptr::NonNull;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    VirtAddr, PhysAddr,
};
use spin::Mutex;

pub fn init(_boot_info: &bootloader_api::info::BootInfo) {
    // TODO: parse memory map & set up frame allocator
    // AIKLE: placeholder — expose memory telemetry to AI daemon
}

pub struct FrameAlloc; // AIKLE: placeholder

unsafe impl FrameAllocator<Size4KiB> for FrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        // AIKLE: placeholder — use AI-guided policy in future
        None
    }
}

static HEAP_LOCK: Mutex<()> = Mutex::new(());

pub fn kmalloc(size: usize) -> NonNull<u8> {
    let _g = HEAP_LOCK.lock();
    // AIKLE: placeholder — delegate to allocator, feed telemetry
    NonNull::dangling()
}
