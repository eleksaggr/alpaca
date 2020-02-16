use super::StackFrame;

use io::{Io, Pio};

use interrupt::driver::PIC;

pub extern "x86-interrupt" fn timer(_frame: &mut StackFrame) {
    logln!("Timer interrupt triggered!");

    PIC.lock().acknowledge(1);
}

pub extern "x86-interrupt" fn keyboard(_frame: &mut StackFrame) {
    println!("Key pressed!");

    let port = Pio::new(0x60);
    println!("Scan code: {}", port.read());

    PIC.lock().acknowledge(2);
}
