use core::marker::PhantomData;

use super::Io;

pub struct Pio<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> Pio<T> {
    pub const fn new(port: u16) -> Self {
        Self {
            port: port,
            value: PhantomData,
        }
    }
}

impl Io for Pio<u8> {
    type Value = u8;

    #[inline(always)]
    fn read(&self) -> u8 {
        let mut value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}
