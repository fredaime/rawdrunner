#![allow(dead_code)]
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use core::ptr::NonNull;
use spin::Mutex;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

/// Simple frame allocator initialized from the bootloader memory map.
pub struct FrameAlloc {
    memory_map: &'static MemoryRegions,
    next: usize,
}

unsafe impl Send for FrameAlloc {}
unsafe impl Sync for FrameAlloc {}

impl FrameAlloc {
    /// Create a new frame allocator from the bootloader memory map.
    pub unsafe fn new(memory_map: &'static MemoryRegions) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.memory_map
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .flat_map(|r| {
                let start = PhysAddr::new(r.start);
                let end = PhysAddr::new(r.end - 1);
                let start_frame = PhysFrame::containing_address(start);
                let end_frame = PhysFrame::containing_address(end);
                PhysFrame::range_inclusive(start_frame, end_frame)
            })
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next)?;
        self.next += 1;
        Some(frame)
    }
}

static HEAP_LOCK: Mutex<()> = Mutex::new(());
static mut HEAP: [u8; 64 * 4096] = [0; 64 * 4096];
static mut HEAP_OFF: usize = 0;

/// Very small bump allocator used for kernel allocations.
pub fn kmalloc(size: usize) -> NonNull<u8> {
    let _g = HEAP_LOCK.lock();
    let size = (size + 7) & !7; // align to 8 bytes
    unsafe {
        if HEAP_OFF + size > HEAP.len() {
            panic!("kmalloc out of memory");
        }
        let ptr = HEAP.as_mut_ptr().add(HEAP_OFF);
        HEAP_OFF += size;
        NonNull::new(ptr).unwrap()
    }
}

static FRAME_ALLOCATOR: Mutex<Option<FrameAlloc>> = Mutex::new(None);

pub fn init(boot_info: &bootloader_api::info::BootInfo) {
    let regions: &'static MemoryRegions = unsafe { &*(&boot_info.memory_regions as *const _) };
    let mut alloc = FRAME_ALLOCATOR.lock();
    *alloc = Some(unsafe { FrameAlloc::new(regions) });
    // Expose memory statistics via println (telemetry hook)
    let usable = boot_info
        .memory_regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .count();
    crate::println!("[memory] {} usable regions detected", usable);
}

/// Obtain a mutable reference to the global frame allocator.
pub fn frame_allocator() -> spin::MutexGuard<'static, Option<FrameAlloc>> {
    FRAME_ALLOCATOR.lock()
}
