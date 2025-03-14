use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

/// Custom Allocator for the `mm` compiler
pub struct MmAllocator;

unsafe impl GlobalAlloc for MmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
