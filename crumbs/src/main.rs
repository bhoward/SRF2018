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
mod svc;

pub use oom::rust_oom;

use log::*;
use svc::*;
use heap::*;

use cortex_a::asm;

use alloc::boxed::PinBox;
use alloc::string::String;

const MMIO_BASE: u32 = 0x3F00_0000;

#[global_allocator]
static GLOBAL: CrumbsAllocator = CrumbsAllocator;

fn main() {
    log_init();
    svc_init();
    heap_init();

    let lfb_box = PinBox::new(lfb::Lfb::new().expect("unable to construct frame buffer"));
    let window_manager = window_manager::WindowManager::new(lfb_box);
    
    let window = window::Window::new("Test Window", 10, 20, 250, 100);
    let window2 = window::Window::new("Test Window 2", 100, 70, 280, 150);
    let window3 = window::Window::new("Oh look, another one", 400, 300, 300, 150);

    let windows = vec![window, window2, window3];

    window_manager.show(windows);

    window_manager.test();

    // log_heap();

    log("About to send a message\n");

    call_svc1("hello");

    log("Done\n");

    loop {} // never return...
}

#[no_mangle]
pub extern "C" fn exc_handler(exc_type: u32, esr: u32, x2: u64) {
    let ec = (esr >> 26);

    if (exc_type & 0x3) == 0 && ec == 0x15 {
        // SVC call
        let op = esr & 0xFFFF; // immediate argument
        log("SVC ");
        log_hex(op);
        log(" handled\n");

        log("X2 is ");
        log_hex(x2 as u32);
        log("\n");

        let msg = String::from(unsafe { *(x2 as *const &str) });
        log("Message is ");
        log(&msg);
        log("\n");
    } else {
        log("In exc_handler: type = ");
        log_hex(exc_type);
        log(", esr = ");
        log_hex(esr);
        log("\n");
    }
}
