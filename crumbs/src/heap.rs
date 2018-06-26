use heap;
use log::*;

pub const SYSTEM_BITS: usize = usize::min_value().count_zeros() as usize;
//pub const SYSTEM_BITS: usize = 32;

extern {
    static mut _end : u8;
}

pub struct Heap {
    pub free_lists: [*mut u8; SYSTEM_BITS],
}

impl Heap {
    pub fn new() -> Heap {
        let k_end = unsafe { &mut _end as *mut u8 };
        let h_end = 0x3EFFFFFF as *mut u8;

        log("Heap start: ");
        log_hex(k_end as *mut _ as u32);
        log("\n");
        log("Heap end: ");
        log_hex(h_end as *mut _ as u32);
        log("\n");

        let free_lists = [0 as *mut u8; SYSTEM_BITS];

        let heap_start = (((k_end as usize - 1) / 8 + 1) * 8) as *mut u8;
        let heap_size = unsafe { h_end.offset_from(heap_start) as usize + 1 };

        log("Real start: ");
        log_hex(heap_start as u32);
        log("\n");
        log("Heap size: ");
        log_hex(heap_size as u32);
        log("\n");

        let mut heap = Heap { free_lists };

        heap.free_blocks(heap_start, 8, heap_size);

        heap.log_heap();

        heap
    }

    pub fn free(&mut self, block: *mut u8, free_size: usize) {
        if free_size >= 8 {
            let block_size: usize = usize::next_power_of_two(free_size / 2 + 1);
            let block_end: *mut u8 = unsafe { block.offset(block_size as isize) };
            let free_lists_index: usize = usize::trailing_zeros(block_size) as usize;

            let buddy: *mut u8 = (block as usize ^ block_size) as *mut u8;
            let mut prev: *mut *mut u8 = &mut self.free_lists[free_lists_index];

            unsafe {
                while *prev > block && *prev != buddy {
                    prev = *prev as *mut *mut u8;
                }

                if *prev == buddy && !buddy.is_null() {
                    *prev = *(buddy as *mut *mut u8);
                    let smaller = if block < buddy {block} else {buddy};
                    self.free(smaller, block_size * 2);
                } else {
                    let p = block as *mut *mut u8;
                    *p = *prev;
                    *prev = block;
                }
            }

            if (free_size - block_size) >= 8 {
                self.free(block_end, free_size - block_size);
            } // else shouldn't happen -- allocate in multiples of 8
        }
    }

    fn free_blocks(&mut self, start: *mut u8, curr_size: usize, total_size: usize) {  // for init only, consider better name
        if total_size > 0 {
            if (start as usize & curr_size) != 0 {
                let size = if curr_size < total_size {curr_size} else {total_size};
                self.free(start, size);
                let next = unsafe { start.offset(size as isize) };
                self.free_blocks(next, curr_size * 2, total_size - size);
            } else {
                self.free_blocks(start, curr_size * 2, total_size);
            }
        }
    }

    pub fn alloc(&mut self, req_size: usize) -> *mut u8 {
        let block_size: usize = usize::next_power_of_two(req_size);
        let free_lists_index: usize = usize::trailing_zeros(block_size) as usize;

        let new_block: *mut u8 = self.free_lists[free_lists_index];

        unsafe{ self.free_lists[free_lists_index] = *(new_block as *mut *mut u8); } // put block previously pointed to by the block we're allocing in free_lists
        unsafe{ self.free_blocks(new_block.offset(req_size as isize), 8, block_size - req_size); }

        return new_block;
    }

    pub fn log_heap(&self) {
        log("Size: Node -> ...\n");

        for i in 0 .. SYSTEM_BITS {
            log_hex(2_u32.pow(i as u32));
            log(": ");
            self.log_node(self.free_lists[i]);
        }
    }

    fn log_node(&self, node: *mut u8) {
        if node.is_null() {
            log(" -> none\n");
        } else {
            log(" -> ");
            unsafe { log_hex(node as u32); }
            unsafe { self.log_node(*(node as *mut *mut u8)); }
        }
    }
}
