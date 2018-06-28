#[lang = "oom"]
#[no_mangle]
pub extern fn rust_oom(_: core::alloc::Layout) -> ! {
    panic!("Out of memory.");
}
