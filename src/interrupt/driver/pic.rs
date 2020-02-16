use io::{Io, Pio};

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

pub struct ChainedPic {
    master: Pic,
    slave: Pic,
}

impl ChainedPic {
    pub fn new(master: u16, slave: u16) -> Self {
        Self {
            master: Pic::new(master),
            slave: Pic::new(slave),
        }
    }

    pub fn remap(&mut self, offset: u8) {
        if offset < 0x20 {
            panic!("Attempt to map PIC onto exception range");
        }

        let master_mask = self.master.data.read();
        let slave_mask = self.slave.data.read();

        // Start the initialization sequence.
        self.master.cmd.write(Pic::INIT | Pic::ICW4);
        self.slave.cmd.write(Pic::INIT | Pic::ICW4);

        // Set the vector offsets, to avoid clashing with x86 exceptions.
        self.master.data.write(offset);
        self.slave.data.write(offset + 8);

        // Tell the master that the slave is on IRQ2.
        self.master.data.write(1 << (Irq::Slave as usize));
        // Tell the slave its cascade identity.
        self.slave.data.write(1 << 1);

        // Set the mode for both master and slave.
        self.master.data.write(Pic::MODE);
        self.slave.data.write(Pic::MODE);

        self.master.data.write(master_mask);
        self.slave.data.write(slave_mask);
    }

    pub fn disable(&mut self) {
        self.slave.data.write(0xff);
        self.master.data.write(0xff);
    }

    pub fn acknowledge(&mut self, irq: u8) {
        if irq >= 8 {
            self.slave.cmd.write(Pic::EOI);
        }
        self.master.cmd.write(Pic::EOI);
    }

    pub fn mask(&mut self, irq: u8) {
        if irq > 8 {
            self.slave.mask(irq - 8);
        } else {
            self.master.mask(irq);
        }
    }

    pub fn unmask(&mut self, irq: Irq) {
        if irq as usize > 8 {
            self.slave.unmask(irq as u8 - 8);
        } else {
            self.master.unmask(irq as u8);
        }
    }

    pub fn irr(&mut self) -> u16 {
        self.master.cmd.write(Pic::IRR);
        self.slave.cmd.write(Pic::IRR);

        ((self.slave.cmd.read() as u16) << 8) | (self.master.cmd.read() as u16)
    }

    pub fn isr(&mut self) -> u16 {
        self.master.cmd.write(Pic::ISR);
        self.slave.cmd.write(Pic::ISR);

        ((self.slave.cmd.read() as u16) << 8) | (self.master.cmd.read() as u16)
    }
}

struct Pic {
    cmd: Pio<u8>,
    data: Pio<u8>,
}

impl Pic {
    const INIT: u8 = 0x10;
    const ICW4: u8 = 0x01;
    const MODE: u8 = 0x01;
    const EOI: u8 = 0x20;
    const IRR: u8 = 0x0a;
    const ISR: u8 = 0x0b;

    const fn new(port: u16) -> Self {
        Self {
            cmd: Pio::new(port),
            data: Pio::new(port + 1),
        }
    }

    fn mask(&mut self, irq: u8) {
        self.data.write(self.data.read() | (1 << irq));
    }

    fn unmask(&mut self, irq: u8) {
        self.data.write(self.data.read() & !(1 << irq));
    }
}
