#[panic_implementation]
#[no_mangle]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    logln!("{}", info);
    loop {}
}
