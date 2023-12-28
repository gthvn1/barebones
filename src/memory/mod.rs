pub mod bump_allocator;
pub mod multiboot;

use bump_allocator::BumpAllocator;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

pub unsafe fn init(memory_start: usize, memory_size: usize) {
    ALLOCATOR.inner.lock().init(memory_start, memory_size);
}
