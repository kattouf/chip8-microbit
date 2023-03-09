use core::alloc::Layout;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_heap_allocator() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("Out of memory!");
}
