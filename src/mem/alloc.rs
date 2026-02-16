use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SIZE: usize = 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE]; // gig of ram

pub fn init_heap() {
    unsafe {
        #[allow(static_mut_refs)]
        let heap_start = HEAP.as_mut_ptr();
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }
}
