pub mod exception;

lazy_static! {
    static ref IDT: [Entry; 256] = {
        let mut idt: [Entry; 256] = [Default::default(); 256];

        idt[3] = Entry::with(Handler::Func(exception::breakpoint)).trap();
        idt[8] = Entry::with(Handler::FuncWithError(exception::double));

        logln!("Showing full IDT:");
        for (i,entry) in idt.iter().enumerate() {
            if entry.present() {
                logln!("{}: {:#x?}", i, entry);
            }
        }
        
        idt
    };
}

#[inline(always)]
fn load(ptr: &Pointer) {
    unsafe {
        asm!("lidt ($0)": : "r"(ptr): "memory");
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C,packed)]
pub struct Pointer {
    limit: u16,
    base: u64,
}

pub fn init() {
    let ptr = Pointer {
        limit: (IDT.len() * core::mem::size_of::<Entry>() - 1) as u16,
        base: IDT.as_ptr() as u64,
    };

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

#[derive(Clone, Copy)]
pub enum Handler {
    Func(extern "x86-interrupt" fn(&mut StackFrame)),
    FuncWithError(extern "x86-interrupt" fn(&mut StackFrame, u64)),
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
        let mut entry: Entry = Default::default();
        
        // Set offset with the given address.
        let base: usize;
        match handler {
            Handler::Func(f) => base = f as usize,
            Handler::FuncWithError(f) => base = f as usize,
        }

        entry.offsetl = base as u16;
        entry.offsetm = (base >> 16) as u16;
        entry.offseth = (base >> 32) as u32;

        // Mark the entry as present.
        entry.attributes = 1 << 7;

        // Per default this should be an interrupt and not a trap.
        entry.attributes = entry.attributes | 0xE;

        // Set the correct code selector.
        let cs: u16;
        unsafe {
            asm!("mov $0, cs" : "=r"(cs) : : : "intel", "volatile");
        }
        entry.selector = cs;

        entry
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
        Self {
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
