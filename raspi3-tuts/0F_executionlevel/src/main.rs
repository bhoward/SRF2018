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

extern crate cortex_a;
extern crate raspi3_glue;
extern crate rlibc; // provides memset

#[macro_use]
extern crate register;

mod gpio;
mod mbox;
mod uart;

use cortex_a::regs::currentel::*;

const MMIO_BASE: u32 = 0x3F00_0000;

fn main() {
    let mut mbox = mbox::Mbox::new();
    let uart = uart::Uart::new();

    // set up serial console
    if uart.init(&mut mbox).is_err() {
        return; // If UART fails, abort early
    }

    uart.getc(); // Press a key first before being greeted
    uart.puts("Hello Rustacean!\n");

    let el = (CurrentEL.get() >> 2) & 0x3;

    uart.puts("Current EL is: ");
    uart.hex(el);
    uart.puts("\n");

    // echo everything back
    loop {
        uart.send(uart.getc());
    }
}


