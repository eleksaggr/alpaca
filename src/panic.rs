use interrupt;

#[no_mangle]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    logln!("{}", info);
    interrupt::disable();
    loop {}
}
