use core::alloc::{GlobalAlloc, Layout};
use spin::Mutex;

pub struct BumpAllocator {
    pub inner: Mutex<BumpInternals>,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(BumpInternals::new()),
        }
    }
}

pub struct BumpInternals {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpInternals {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
        self.allocations = 0;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: real allocation. Currently we are just returning the
        // next pointer and incrementing it by the size of the layout.
        let size = match Layout::from_size_align(layout.size(), layout.align()) {
            Ok(l) => l.size(),
            Err(_) => panic!("Invalid layout"),
        };
        let return_ptr = self.inner.lock().next as *mut u8;

        self.inner.lock().next += size;
        return_ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("TODO: deallocating {:?} with layout {:?}", ptr, layout);
    }
}
