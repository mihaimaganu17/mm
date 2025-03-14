use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Maximum number of bytes that `MmAllocator` can allocate until it runs out of memory.
const MEM_POOL_SIZE: usize = 256 * 1024;

pub struct MmAllocator {
    pool: UnsafeCell<[u8; MEM_POOL_SIZE]>,
    // We use `AtomicUsize` because we need to make sure that if 2 threads are trying to allocate
    // at the same time, there does not exist a race condition for accessing the memory pool
    next_free_addr: AtomicUsize,
}

/// static variables need to be shared safely between threads
unsafe impl Sync for MmAllocator {}

/// Custom Allocator for the `mm` compiler
//#[global_allocator]
pub static ALLOCATOR: MmAllocator = MmAllocator {
    pool: UnsafeCell::new([0; MEM_POOL_SIZE]),
    next_free_addr: AtomicUsize::new(0),
};

unsafe impl GlobalAlloc for MmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Compute the amount of memory we need
        let mem_needed = (layout.size() + layout.align() - 1) & !(layout.align() - 1);
        // Check if we have space
        if self.next_free_addr.load(Ordering::Relaxed) + mem_needed > self.pool.get().as_ref().unwrap().len() {
            ptr::null_mut()
        } else {
            self.next_free_addr.fetch_add(mem_needed, Ordering::SeqCst);
            // Get the pointer to the underlying data at the proper offset
            (self.pool.get() as *mut u8).add(mem_needed)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
