use super::StackFrame;

lazy_static! {
    static ref IDT: [Entry; 256] = {
        let mut idt: [Entry; 256] = [Default::default(); 256];

        idt[3] = Entry::with(Handler::Func(exception::breakpoint)).trap();
        idt[8] = Entry::with(Handler::FuncWithError(exception::double));
        idt[32] = Entry::with(Handler::Func(irq::timer));
        idt[33] = Entry::with(Handler::Func(irq::keyboard));

        logln!("Showing full IDT:");
        for (i, entry) in idt.iter().enumerate() {
            if entry.present() {
                logln!("{}: {:#x?}", i, entry);
            }
        }

        idt
    };
}

type Idt = [Entry; 256];

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Pointer {
    limit: u16,
    base: u64,
}

#[inline(always)]
fn load(ptr: &Pointer) {
    unsafe {
        asm!("lidt ($0)": : "r"(ptr): "memory");
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Entry {
    offsetl: u16,
    selector: u16,
    ist: u8,
    attributes: u8,
    offsetm: u16,
    offseth: u32,
    _pad: u32,
}

impl Entry {
    pub fn with(handler: Handler) -> Self {
        // Set offset with the given address.
        let base = match handler {
            Handler::Func(f) => f as usize,
            Handler::FuncWithError(f) => f as usize,
        };

        let mut entry = Entry {
            offsetl: base as u16,
            offsetm: (base >> 16) as u16,
            offseth: (base >> 32) as u32,
            attributes: (1 << 7) | 0xE,
            ..Default::default()
        };

        // Set the correct code selector.
        let cs: u16;
        unsafe {
            asm!("mov $0, cs" : "=r"(cs) : : : "intel", "volatile");
        }
        entry.selector = cs;

        entry
    }

    pub fn interrupt(mut self) -> Self {
        self.attributes = self.attributes | 0xE;
        self
    }

    pub fn trap(mut self) -> Self {
        self.attributes = self.attributes | 0xF;
        self
    }

    pub fn present(&self) -> bool {
        self.attributes & (1 << 7) != 0
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            offsetl: 0,
            selector: 0,
            ist: 0,
            attributes: 0,
            offsetm: 0,
            offseth: 0,
            _pad: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Handler {
    Func(extern "x86-interrupt" fn(&mut StackFrame)),
    FuncWithError(extern "x86-interrupt" fn(&mut StackFrame, u64)),
}
