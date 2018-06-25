use uart;
use mbox;

pub static mut UART: Option<uart::Uart> = None;

pub fn log_init() {
    let mut mbox = mbox::Mbox::new();
    let uart = uart::Uart::new();
    
    // set up serial console
    if uart.init(&mut mbox).is_err() {
        return; // If UART fails, abort early
    }

    unsafe { UART = Some(uart); }
}

pub fn log(msg: &str) {
    unsafe { UART.as_ref().unwrap().puts(msg); }
}

pub fn log_hex(d: u32) {
    unsafe { UART.as_ref().unwrap().hex(d); }
}