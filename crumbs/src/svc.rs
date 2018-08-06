pub fn svc_init() {
    // set up exception handlers here -- code based on
    // https://github.com/bztsrc/raspi3-tutorial/blob/master/11_exceptions/start.S
    // and https://web.stanford.edu/class/cs140e/assignments/3-spawn/

    unsafe {
        asm!("
                ldr     x16, =_vectors
                msr     vbar_el1, x16
                b       1f

            save_and_call:
                stp     x2, x3, [sp, #0x10]
                stp     x4, x5, [sp, #0x20]
                stp     x6, x7, [sp, #0x30]
                stp     x8, x9, [sp, #0x40]
                stp     x10, x11, [sp, #0x50]
                stp     x12, x13, [sp, #0x60]
                stp     x14, x15, [sp, #0x70]
                stp     x16, x17, [sp, #0x80]
                stp     x18, x30, [sp, #0x90]
                bl      exc_handler
                ldp     x2, x3, [sp, #0x10]
                ldp     x4, x5, [sp, #0x20]
                ldp     x6, x7, [sp, #0x30]
                ldp     x8, x9, [sp, #0x40]
                ldp     x10, x11, [sp, #0x50]
                ldp     x12, x13, [sp, #0x60]
                ldp     x14, x15, [sp, #0x70]
                ldp     x16, x17, [sp, #0x80]
                ldp     x18, x30, [sp, #0x90]
                ldp     x0, x1, [sp], #0xa0
                eret

            // important, code has to be properly aligned
                .align 11
            
            _vectors:
                // From same EL with SP_EL0, synchronous
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #0
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_EL0, IRQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #1
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_EL0, FIQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #2
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_EL0, SError
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #3
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_ELx, synchronous
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #4
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_ELx, IRQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #5
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_ELx, FIQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #6
                mrs     x1, esr_el1
                b       save_and_call

                // From same EL with SP_ELx, SError
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #7
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch64, synchronous
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #8
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch64, IRQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #9
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch64, FIQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #10
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch64, SError
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #11
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch32, synchronous
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #12
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch32, IRQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #13
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch32, FIQ
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #14
                mrs     x1, esr_el1
                b       save_and_call

                // From lower EL on aarch32, SError
                .align  7
                stp     x0, x1, [sp, #-0xa0]!
                mov     x0, #15
                mrs     x1, esr_el1
                b       save_and_call

            1:  nop
        " ::: "x16" : "volatile")
    }
}

pub fn call_svc1(arg: &str) {
    unsafe {
        asm!("
            mov     x2, $0
            svc     #1
        " :: "r"(&arg as *const _ as u64) : "x2" : "volatile")
    }
}
