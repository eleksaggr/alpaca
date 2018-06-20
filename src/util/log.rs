use io::devices::serial::SerialPort;
use spin::Mutex;

lazy_static! {
    static ref SERIAL: Mutex<SerialPort> = Mutex::new({
        let mut serial = SerialPort::new(0x3f8);
        serial.init();
        serial
    });
}

macro_rules! logln {
    () => (log!("\n"));
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! log {
    ($($arg:tt)*) => ($crate::util::log::print(format_args!($($arg)*)));
}

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}
