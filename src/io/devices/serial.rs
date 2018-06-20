use io::pio::Pio;
use io::Io;

pub struct SerialPort {
    data: Pio<u8>,
    interrupt: Pio<u8>,
    fifo: Pio<u8>,
    line: Pio<u8>,
    modem: Pio<u8>,
    status: Pio<u8>,
}

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        Self {
            data: Pio::new(port),
            interrupt: Pio::new(port + 1),
            fifo: Pio::new(port + 2),
            line: Pio::new(port + 3),
            modem: Pio::new(port + 4),
            status: Pio::new(port + 5),
        }
    }

    pub fn init(&mut self) {
        self.interrupt.write(0x00);
        self.line.write(0x80);
        self.data.write(0x03);
        self.interrupt.write(0x00);
        self.line.write(0x03);
        self.fifo.write(0xc7);
        self.modem.write(0x0b);
    }

    pub fn read(&self) -> u8 {
        while self.status.read() & 1 == 0 {}
        self.data.read()
    }

    pub fn write(&mut self, value: u8) {
        while self.status.read() & 0x20 == 0 {}
        self.data.write(value);
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}
