extern {
    static mut _end : u8;
}

pub struct Heap {
    pub free_list_starts: [isize; 36],
    pub k_end: *mut u8,
    pub h_end: *mut u8,
}

impl Heap {
    pub fn new() -> Heap {
        let k_end = unsafe { &mut _end as *mut u8 };
        let h_end = 0x3EFFFFFF as *mut u8;
        Heap {free_list_starts: [0; 36], k_end, h_end}
    }
}
