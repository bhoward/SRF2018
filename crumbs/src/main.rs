/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 * Copyright (c) 2018 Brian Howard <bhoward@depauw.edu>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![no_std]
#![feature(asm)]
#![feature(const_int_ops)]
#![feature(ptr_offset_from)]
#![feature(allocator_api, heap_api)]
#![feature(alloc, extern_prelude, lang_items)]

extern crate raspi3_glue;
extern crate volatile_register;
extern crate rlibc;

#[macro_use]
extern crate alloc;

mod gpio;
mod mbox;
mod uart;
mod lfb;
mod font;
mod window;
mod colors;
mod window_manager;
mod heap;
mod log;
mod oom;

use colors::*;
use log::*;
use heap::*;

const MMIO_BASE: u32 = 0x3F00_0000;

#[global_allocator]
static GLOBAL: CrumbsAllocator = CrumbsAllocator;

pub static mut HEAP: Heap = Heap {
    free_lists: [0 as *mut u8; SYSTEM_BITS]
};

fn main() {
    log_init();
    unsafe { HEAP.init(); }

    // set up linear frame buffer
    let lfb = lfb::Lfb::new().expect("unable to construct frame buffer");

    let window_manager = window_manager::WindowManager::new();
    window_manager.fill_bg(&lfb);

    //lfb.print(10, 5, "Hello Rustacean (Castlemakers if you prefer)!", RED_PIXEL);

    //lfb.line();
    
    let window = window::Window::new("Test Window", 10, 20, 250, 100);
    let window2 = window::Window::new("Test Window 2", 100, 70, 280, 150);
    window.show(&lfb);

/*
    log("Heap init...\n");
    let mut heap = heap::Heap::new();

    log("Alloc test_block...\n");
    let test_block = heap.alloc(2398) as *mut u32;

    unsafe { *(test_block) = 0xCAFEBABE };

    log("\n");
    heap.log_heap();
    log("\n");

    unsafe { log_hex(*test_block); }
    log("\n");

    let new_test_block = heap.realloc(test_block as *mut u8, 2398, 344212) as *mut u32;
    heap.log_heap();
    
    unsafe { log_hex(*new_test_block); }

    log("\n");
*/

    window2.show(&lfb);

    {
        use alloc::boxed::Box;
        let b = Box::new(42);
        log("Box contents: ");
        log_hex(*b);
        log("\n");
        let bp = Box::into_raw(b);
        log("Box address: ");
        log_hex(bp as u32);
        log("\n");

        unsafe { HEAP.log_heap(); }

        let b = unsafe { Box::from_raw(bp) };
    }

    unsafe { HEAP.log_heap(); }

    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;
    for i in &vec_test {
        log_hex(i as *const _ as u32);
        log("\n");
    }
    unsafe { HEAP.log_heap(); }


/*
    // echo everything back
    loop {
        uart.send(uart.getc());
    }
*/
}


