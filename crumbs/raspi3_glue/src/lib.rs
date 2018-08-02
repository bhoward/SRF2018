/*
 * MIT License
 *
 * Copyright (c) 2018 Jorge Aparicio
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

#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate cortex_a;
extern crate panic_abort;
extern crate r0;

#[lang = "start"]
extern "C" fn start<T>(user_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    user_main().report() as isize
}

#[lang = "termination"]
trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

unsafe fn reset() -> ! {
    use core::ptr;

    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;

        // Boundaries of the .bss section
        static mut __bss_start: u32;
        static mut __bss_end: u32;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    main(0, ptr::null());

    loop {}
}

/// Entrypoint of the RPi3.
///
/// Parks all cores except core0, and then jumps to the internal
/// `reset()` function, which will call the user's `main()` after
/// initializing the `bss` section.
#[link_section = ".text.boot"]
#[no_mangle]
pub extern "C" fn _boot_cores() -> ! {
    use cortex_a::{
        asm,
        regs::mpidr_el1::*,
        regs::sp::*,
        regs::currentel::*,
        regs::sp_el1::*,
        regs::cnthctl_el2::*,
        regs::cntvoff_el2::*,
        regs::hcr_el2::*,
        regs::cpacr_el1::*,
        regs::scr_el3::*,
        regs::spsr_el3::*,
        regs::spsr_el2::*
    };

    match MPIDR_EL1.get() & 0x3 {
        0 => {
            let el = (CurrentEL.get() >> 2) & 0x3;
            if el == 3 {
                // this usually won't happen, unless requested in config.txt
                // first change exception level to EL2

                // enable AArch64 in EL2
                SCR_EL3.set(0x5B1); // RW+HCE+SMD+NS

                SPSR_EL3.set(0x3C9); // D+A+I+F+EL2h
                
                // ELR_EL3.set(???); // address of code to "return" to
                // asm::eret();
                // TODO clean this up
                unsafe {
                    asm!("
                        adr x2, 2f
                        msr elr_el3, x2
                        eret
                    2:  nop
                    " ::: "x2" : "volatile")
                }
            }
            
            if el >= 2 {
                SP_EL1.set(0x80_000);

                // enable CNTP for EL1
                CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCTEN::SET + CNTHCTL_EL2::EL1PCEN::SET);
                CNTVOFF_EL2.set(0);

                // enable AArch64 in EL1
                HCR_EL2.modify(HCR_EL2::RW::SET + HCR_EL2::SWIO::SET);

                // enable floating-point and SIMD in EL0/1
                CPACR_EL1.modify(CPACR_EL1::FPEN.val(3));

                SPSR_EL2.set(0x3C4); // D+A+I+F+EL1t

                // change exception level to EL1
                // ELR_EL2.set(???); // address of code to "return" to
                // asm::eret();
                // TODO clean this up
                unsafe {
                    asm!("
                        adr x2, 1f
                        msr elr_el2, x2
                        eret
                    1:  nop
                    " ::: "x2" : "volatile")
                }
            }

            SP.set(0x80_000);
            unsafe { reset() }
        }
        _ => loop {
            // if not core0, infinitely wait for events
            asm::wfe();
        }
    }
}
