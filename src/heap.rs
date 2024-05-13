use embedded_alloc::Heap;

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

pub unsafe fn init_global_heap() {
    const HEAP_SIZE: usize = 1024;
    defmt::debug!("Allocate heap of {} bytes", HEAP_SIZE);
    use core::mem::MaybeUninit;
    // Of course HEAP_MEM is mutable, but declaring it as immutable compiler will place it in FLASH memory
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use alloc::vec::Vec;
    use defmt::assert;

    #[test]
    fn check_vec() {
        let vec = Vec::<u8>::new();
        assert!(vec.len() == 0);
    }
}
