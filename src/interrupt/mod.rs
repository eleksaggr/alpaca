pub mod exception;

lazy_static! {
    static ref IDT: [Entry; 256] = {
        let mut idt: [Entry; 256] = [Default::default(); 256];

        idt[3].set(exception::breakpoint as usize, false);
        idt[8].set(exception::double as usize, true);

        idt
    };
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
    logln!("Loading IDT at {} with size {}.", ptr.base, ptr.limit);
    load(&ptr);
}

pub fn load(ptr: &Pointer) {
    unsafe {
        asm!("lidt ($0)": : "r"(ptr): "memory");
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Entry {
    offsetl: u16,
    selector: u16,
    _padding1: u8,
    attributes: u8,
    offsetm: u16,
    offseth: u32,
    _padding2: u32,
}

impl Entry {
    pub fn set(&mut self, addr: usize, trap: bool) {
        self.offsetl = addr as u16;
        self.offsetm = (addr >> 16) as u16;
        self.offseth = (addr >> 32) as u32;

        self.attributes = 1 << 7;
        if trap {
            self.attributes = self.attributes | 0xF;
        } else {
            self.attributes = self.attributes | 0xE;
        }
        self.attributes = 0xE | (1 << 7);

        let cs: u16;
        unsafe {
            asm!("mov $0, cs" : "=r"(cs) : : : "intel");
        }
        self.selector = cs;
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            offsetl: 0,
            selector: 0,
            _padding1: 0,
            attributes: 0,
            offsetm: 0,
            offseth: 0,
            _padding2: 0,
        }
    }
}
