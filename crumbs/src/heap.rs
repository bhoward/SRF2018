use heap;
use log::*;
use ::HEAP;

use core::alloc::{GlobalAlloc, Layout};

pub const SYSTEM_BITS: usize = usize::min_value().count_zeros() as usize;
//pub const SYSTEM_BITS: usize = 32;

extern {
    static mut _end : u8;
}

pub struct Heap {
    pub free_lists: [*mut u8; SYSTEM_BITS],
}

pub struct CrumbsAllocator;

unsafe impl GlobalAlloc for CrumbsAllocator {
    // TODO need to synchronize access to HEAP...
    // TODO use the alignment field of layout?
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.allocate(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.free(ptr, layout.size());
    }
}

impl Heap {
    pub fn init(&mut self) {
        let k_end = unsafe { &mut _end as *mut u8 };
        let h_end = 0x3EFFFFFF as *mut u8;

        let heap_start = heap::round_to_eights(k_end as usize) as *mut u8;
        let heap_size = unsafe { h_end.offset_from(heap_start) as usize + 1 };

        self.free_unaligned(heap_start, 8, heap_size);

        self.log_heap();
    }

    pub fn free(&mut self, block: *mut u8, free_size: usize) {
        let free_size = heap::round_to_eights(free_size);
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

        if (free_size - block_size) > 0 {
            self.free(block_end, free_size - block_size);
        } // else shouldn't happen -- allocate in multiples of 8
    }

    fn free_unaligned(&mut self, start: *mut u8, curr_size: usize, total_size: usize) {  // for init only, consider better name
        if total_size > 0 {
            if (start as usize & curr_size) != 0 {
                let size = if curr_size < total_size {curr_size} else {total_size};
                self.free(start, size);
                let next = unsafe { start.offset(size as isize) };
                self.free_unaligned(next, curr_size * 2, total_size - size);
            } else {
                self.free_unaligned(start, curr_size * 2, total_size);
            }
        }
    }

    pub fn allocate(&mut self, req_size: usize) -> *mut u8 {
        let req_size: usize = heap::round_to_eights(req_size);
        let block_size: usize = usize::next_power_of_two(req_size);
        let mut free_lists_index: usize = usize::trailing_zeros(block_size) as usize;

        while free_lists_index < SYSTEM_BITS && self.free_lists[free_lists_index].is_null() {
            free_lists_index += 1;
        }

        if free_lists_index == SYSTEM_BITS {
            return 0 as *mut u8;
        }

        let new_block: *mut u8 = self.free_lists[free_lists_index];

        unsafe{ self.free_lists[free_lists_index] = *(new_block as *mut *mut u8); } // put block previously pointed to by the block we're allocing in free_lists
        unsafe{ self.free_unaligned(new_block.offset(req_size as isize), 8, (1 << free_lists_index) - req_size); } //(1 << free_lists_index) recompute block_size

        return new_block;
    }
/*
    pub fn realloc(&mut self, old_block: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
        use core::{ptr, cmp};

        let new_block = self.allocate(new_size);
        unsafe { ptr::copy(old_block, new_block, cmp::min(old_size, new_size)); }
        self.free(old_block, old_size);

        new_block
    }
*/
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
            log_hex(node as u32);
            unsafe { self.log_node(*(node as *mut *mut u8)); }
        }
    }
}

fn round_to_eights(size: usize) -> usize {
    (((size - 1) / 8) + 1) * 8
}
