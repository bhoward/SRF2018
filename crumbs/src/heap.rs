extern {
    static _end : *mut u8;
    static __bss_start : *mut u8;
    static __bss_end : *mut u8;
}

pub struct Heap {
    pub free_list_starts: [isize; 36],
    pub k_end: *mut u8,
    pub h_end: *mut u8,
    pub bss_start: *mut u8,
    pub bss_end: *mut u8,
}

impl Heap {
    pub fn new() -> Heap {
        let k_end = unsafe { _end };
        let h_end = 0x3EFFFFFF as *mut u8;
        let bss_start = unsafe { __bss_start };
        let bss_end = unsafe { __bss_end };
        Heap {free_list_starts: [0; 36], k_end, h_end, bss_start, bss_end}
    }
}
