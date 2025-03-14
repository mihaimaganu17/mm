use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

pub struct MmAllocator;

/// Custom Allocator for the `mm` compiler
#[global_allocator]
static Allocator: MmAllocator = MmAllocator;

unsafe impl GlobalAlloc for MmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
