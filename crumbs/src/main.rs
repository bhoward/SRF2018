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
#![feature(pin)]

extern crate cortex_a;
extern crate raspi3_glue;
extern crate rlibc;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate register;

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

pub use oom::rust_oom;

use log::*;
use heap::*;

use alloc::boxed::PinBox;

const MMIO_BASE: u32 = 0x3F00_0000;

#[global_allocator]
static GLOBAL: CrumbsAllocator = CrumbsAllocator;

pub static mut HEAP: Heap = Heap {
    free_lists: [0 as *mut u8; SYSTEM_BITS]
};

fn main() {
    log_init();
    unsafe { HEAP.init(); }

    let lfb_box = PinBox::new(lfb::Lfb::new().expect("unable to construct frame buffer"));
    let window_manager = window_manager::WindowManager::new(lfb_box);
    
    let window = window::Window::new("Test Window", 10, 20, 250, 100);
    let window2 = window::Window::new("Test Window 2", 100, 70, 280, 150);
    let window3 = window::Window::new("Oh look, another one", 400, 300, 300, 150);

    let windows = vec![window, window2, window3];

    window_manager.show(windows);

    window_manager.test();

    unsafe { HEAP.log_heap(); }
}


