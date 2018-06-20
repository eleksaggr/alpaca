pub mod devices;
pub mod pio;
#[macro_use]
pub mod vga;

pub trait Io {
    type Value;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);
}
