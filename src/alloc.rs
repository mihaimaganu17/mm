use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Maximum number of bytes that `MmAllocator` can allocate until it runs out of memory.
const MEM_POOL_SIZE: usize = 256 * 10024;

pub struct MmAllocator {
    pool: UnsafeCell<[u8; MEM_POOL_SIZE]>,
    // We use `AtomicUsize` because we need to make sure that if 2 threads are trying to allocate
    // at the same time, there does not exist a race condition for accessing the memory pool
    remaining: AtomicUsize,
}

/// static variables need to be shared safely between threads
unsafe impl Sync for MmAllocator {}

/// Custom Allocator for the `mm` compiler
#[global_allocator]
pub static ALLOCATOR: MmAllocator = MmAllocator {
    pool: UnsafeCell::new([0; MEM_POOL_SIZE]),
    remaining: AtomicUsize::new(MEM_POOL_SIZE),
};

unsafe impl GlobalAlloc for MmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let mask = !(layout.align() - 1);

        let mut allocated = 0;
        // Check if we have space
        if self.remaining.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |mut remaining| {
                // If there is not enough space, we return
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= mask;
                allocated = remaining;
                Some(remaining)
            }
        ).is_err() {
            return ptr::null_mut();
        }

        // Get the pointer to the underlying data at the proper offset
        (self.pool.get() as *mut u8).add(allocated)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
