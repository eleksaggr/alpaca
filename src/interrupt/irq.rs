use super::StackFrame;

use io::{Io, Pio};

use interrupt::driver::PIC;
use interrupt::Irq;
use io::devices::keyboard::Keyboard;

pub extern "x86-interrupt" fn timer(_frame: &mut StackFrame) {
    logln!("Timer interrupt triggered!");

    PIC.lock().acknowledge(Irq::Timer);
}

pub extern "x86-interrupt" fn keyboard(_frame: &mut StackFrame) {
    let port = Pio::new(0x60);

    let scan = port.read() as u16;
    if scan > 0x80 {
        print!("{}", Keyboard::convert(scan));
    }

    PIC.lock().acknowledge(Irq::Keyboard);
}
