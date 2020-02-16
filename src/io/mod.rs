pub mod devices;
mod pio;
#[macro_use]
pub mod vga;

pub use self::pio::Pio;

pub trait Io {
    type Value;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);
}
