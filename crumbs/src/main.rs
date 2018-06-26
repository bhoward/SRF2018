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

extern crate raspi3_glue;
extern crate volatile_register;

const MMIO_BASE: u32 = 0x3F00_0000;

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

use colors::*;
use log::*;

fn main() {
    log_init();

    // set up linear frame buffer
    let lfb = lfb::Lfb::new().expect("unable to construct frame buffer");

    let window_manager = window_manager::WindowManager::new();
    window_manager.fill_bg(&lfb);

    //lfb.print(10, 5, "Hello Rustacean (Castlemakers if you prefer)!", RED_PIXEL);

    //lfb.line();
    
    let window = window::Window::new("Test Window", 10, 20, 250, 100);
    let window2 = window::Window::new("Test Window 2", 100, 70, 280, 150);
    window.show(&lfb);
    window2.show(&lfb);

    let mut heap = heap::Heap::new();

    log("Heap start: ");
    log_hex(heap.k_end as *mut _ as u32);
    log("\n");
    log("Heap end: ");
    log_hex(heap.h_end as *mut _ as u32);
    log("\n");

    let k_end = heap.k_end;
    let h_end = heap.h_end;

    unsafe { heap.free(k_end, h_end.offset_from(k_end) as usize); }
    heap.log_heap();

/*
    // echo everything back
    loop {
        uart.send(uart.getc());
    }
*/
}

