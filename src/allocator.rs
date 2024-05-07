use allocator_api2::alloc::{AllocError, Allocator};
use redis_custom_allocator::MemoryConsumption;
use std::{
    alloc::{Layout, System},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct TrackingAllocator {
    allocated: AtomicUsize,
}

impl TrackingAllocator {
    pub fn new(initial: usize) -> Self {
        Self {
            allocated: AtomicUsize::new(initial),
        }
    }
}

unsafe impl Allocator for TrackingAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        System.allocate(layout).map_err(|_| AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
        System.deallocate(ptr, layout);
    }
}

impl MemoryConsumption for TrackingAllocator {
    fn memory_consumption(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }
}
