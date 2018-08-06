pub fn svc_init() {
    // set up exception handlers here -- code based on https://github.com/bztsrc/raspi3-tutorial/blob/master/11_exceptions/start.S

    unsafe {
        asm!("
                ldr     x16, =_vectors
                msr     vbar_el1, x16
                b       1f

            // important, code has to be properly aligned
                .align 11
            
            // TODO figure out better which registers to preserve
            _vectors:
                // synchronous
                .align  7
                stp     x0, x1, [sp, #-16]!
                stp     x9, x10, [sp, #-16]!
                stp     x11, x30, [sp, #-16]!
                mov     x0, #0
                mrs     x1, esr_el1
                bl      exc_handler
                ldp     x11, x30, [sp], #16
                ldp     x9, x10, [sp], #16
                ldp     x0, x1, [sp], #16
                eret

                // IRQ
                .align  7
                stp     x0, x1, [sp, #-16]!
                stp     x9, x10, [sp, #-16]!
                stp     x11, x30, [sp, #-16]!
                mov     x0, #1
                mrs     x1, esr_el1
                bl      exc_handler
                ldp     x11, x30, [sp], #16
                ldp     x9, x10, [sp], #16
                ldp     x0, x1, [sp], #16
                eret

                // FIQ
                .align  7
                stp     x0, x1, [sp, #-16]!
                stp     x9, x10, [sp, #-16]!
                stp     x11, x30, [sp, #-16]!
                mov     x0, #2
                mrs     x1, esr_el1
                bl      exc_handler
                ldp     x11, x30, [sp], #16
                ldp     x9, x10, [sp], #16
                ldp     x0, x1, [sp], #16
                eret

                // SError
                .align  7
                stp     x0, x1, [sp, #-16]!
                stp     x9, x10, [sp, #-16]!
                stp     x11, x30, [sp, #-16]!
                mov     x0, #3
                mrs     x1, esr_el1
                bl      exc_handler
                ldp     x11, x30, [sp], #16
                ldp     x9, x10, [sp], #16
                ldp     x0, x1, [sp], #16
                eret

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
