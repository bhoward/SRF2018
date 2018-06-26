use log::*;

pub const SYSTEM_BITS: usize = usize::min_value().count_zeros() as usize;
//pub const SYSTEM_BITS: usize = 32;

extern {
    static mut _end : u8;
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
        let free_lists = [0 as *mut u8; SYSTEM_BITS];        

        Heap {free_lists, k_end, h_end}
    }

    pub fn free(&mut self, block: *mut u8, free_size: usize) {
        if free_size >= 8 {
            log_hex(block as u32);
            log(": ");
            log_hex(free_size as u32);
            log("\n");

            let block_size: usize = usize::next_power_of_two(free_size / 2);
            let block_end = unsafe { block.offset(block_size as isize) };
            let free_lists_index = usize::trailing_zeros(block_size);

            /*
            log("\n");
            log_hex((free_size - block_size) as u32);
            log("\n");
            */

            //log_hex(free_lists_index);

            let p = block as *mut *mut u8;
            unsafe { *p = self.free_lists[free_lists_index as usize] };
            self.free_lists[free_lists_index as usize] = block;

            if (free_size - block_size) >= 8 {
                self.free(block_end as *mut u8, free_size - block_size);
            }
        }
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
            unsafe { self.log_node(*node as *mut u8); }
        }
    }
}
