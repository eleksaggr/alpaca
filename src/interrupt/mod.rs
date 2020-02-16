pub mod driver;
pub mod exception;
pub mod idt;
pub mod irq;

use interrupt::driver::PIC;

pub fn init() {
    let ptr = Pointer {
        limit: (IDT.len() * core::mem::size_of::<Entry>() - 1) as u16,
        base: IDT.as_ptr() as u64,
    };

    PIC.lock().init(0x20, 0x28);

    for i in 0..16 {
        PIC.lock().mask(i);
    }
    PIC.lock().unmask(1);

    // This requires an unsafe block now?
    unsafe {
        logln!("Loading IDT at {:#x} with size {:#x}.", ptr.base, ptr.limit);
    }
    load(&ptr);
}

#[derive(Clone, Copy, Debug)]
pub struct StackFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}

pub fn enable() {
    unsafe {
        asm!("sti" : : : : "intel", "volatile");
    }
}

pub fn disable() {
    unsafe {
        asm!("cli" : : : : "intel", "volatile");
    }
}
