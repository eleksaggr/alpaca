pub mod driver;
pub mod exception;
pub mod idt;
pub mod irq;

use interrupt::driver::PIC;

#[derive(Copy, Clone, Debug)]
pub enum Irq {
    Timer = 0,
    Keyboard = 1,
    Slave = 2,
    Floppy = 6,
    RTC = 8,
    ACPI = 9,
    Mouse = 12,
    CoProcessor = 13,
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt: idt::Idt = [Default::default(); 256];

        idt[3] = idt::Entry::with(idt::Handler::Func(exception::breakpoint)).trap();
        idt[8] = idt::Entry::with(idt::Handler::FuncWithError(exception::double));
        idt[32] = idt::Entry::with(idt::Handler::Func(irq::timer));
        idt[33] = idt::Entry::with(idt::Handler::Func(irq::keyboard));

        logln!("Showing full IDT:");
        for (i, entry) in idt.iter().enumerate() {
            if entry.present() {
                logln!("{}: {:#x?}", i, entry);
            }
        }

        idt
    };
}

pub fn init() {
    let ptr = idt::Pointer {
        limit: (IDT.len() * core::mem::size_of::<idt::Entry>() - 1) as u16,
        base: IDT.as_ptr() as u64,
    };

    // Remap PIC to interrupt 0x20.
    PIC.lock().remap(0x20);

    // Unmask the keyboard interrupt.
    PIC.lock().unmask(Irq::Keyboard);

    // This requires an unsafe block now?
    unsafe {
        logln!("Loading IDT at {:#x} with size {:#x}.", ptr.base, ptr.limit);
    }

    idt::load(&ptr);
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
