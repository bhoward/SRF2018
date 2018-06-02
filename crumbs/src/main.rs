/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
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

extern crate raspi3_glue;
extern crate volatile_register;

const MMIO_BASE: u32 = 0x3F00_0000;

mod gpio;
mod mbox;
mod uart;
mod lfb;
mod font;

fn main() {
    let uart = uart::MiniUart::new();

     // set up serial console
    uart.init();

    uart.puts("Before new Lfb");

    // set up linear frame buffer
    let lfb = lfb::Lfb::new().expect("unable to construct frame buffer");

    uart.puts("After new Lfb");

    if lfb.width == 1024 {
        uart.puts("1024 cats on logs");
    } else {
        uart.puts("no cats 4 u");
    }
    if lfb.pitch == 4096 {
        uart.puts("pitch is another word for soot I think");
    } else {
        uart.puts("but maybe it's not");
    }

    lfb.print(10, 5, "Hello Rustacean (Castlemakers if you prefer)!");

    lfb.line();

    // echo everything back
    loop {
        uart.send(uart.getc());
    }
}
