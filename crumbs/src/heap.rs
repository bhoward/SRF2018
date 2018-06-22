use uart;

pub const SYSTEM_BITS: usize = usize::min_value().count_zeros() as usize;

extern {
<<<<<<< HEAD
    static _end : *mut u8;
=======
    static mut _end : u8;
>>>>>>> 0b534d1717dca191b93a8d087dd5566803f3d144
}

pub struct Heap {
    pub free_lists: [*mut u8; SYSTEM_BITS],
    pub k_end: *mut u8,
    pub h_end: *mut u8,
}

impl Heap {
    pub fn new() -> Heap {
        let k_end = unsafe { &mut _end as *mut u8 };
        let h_end = 0x3EFFFFFF as *mut u8;
<<<<<<< HEAD
        let free_lists = [0 as *mut u8; SYSTEM_BITS];        

        Heap {free_lists, k_end, h_end}
    }

    pub fn free(&mut self, block: *mut u8, free_size: usize) {
        let block_size: usize = usize::next_power_of_two(free_size / 2);
        let block_end = unsafe { block.offset(block_size as isize) };
        let free_lists_index = usize::count_zeros(block_size);

        let p = block as *mut *mut u8;
        unsafe { *p = self.free_lists[free_lists_index as usize] };
        self.free_lists[free_lists_index as usize] = block;

        if (free_size > block_size) {
            self.free(block_end as *mut u8, free_size - block_size);
        }
=======
        Heap {free_list_starts: [0; 36], k_end, h_end}
>>>>>>> 0b534d1717dca191b93a8d087dd5566803f3d144
    }
}
