use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        Writer {
            column: 0,
            foreground: Color::White,
            background: Color::Black,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    });
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::print(format_args!($($arg)*)));
}

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Character(u8, u8);

type Buffer = [[Character; WIDTH]; HEIGHT];

pub struct Writer {
    column: usize,
    foreground: Color,
    background: Color,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write(&mut self, b: u8) {
        match b {
            b'\n' => self.shift(),
            b => {
                if self.column == WIDTH {
                    self.shift();
                }

                let row = HEIGHT - 1;
                let col = self.column;

                let color = (self.background as u8) << 4 | (self.foreground as u8);
                self.buffer[row][col] = Character(b, color);
                self.column += 1;
            }
        }
    }

    fn shift(&mut self) {
        for r in 1..HEIGHT {
            for c in 0..WIDTH {
                self.buffer[r - 1][c] = self.buffer[r][c];
            }
        }

        for i in 0..WIDTH {
            let color = (self.background as u8) << 4 | (self.foreground as u8);
            self.buffer[HEIGHT - 1][i] = Character(b' ', color);
        }

        self.column = 0;
    }

    pub fn clear(&mut self) {
        for _ in 0..(WIDTH * HEIGHT) {
            self.write(b' ');
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.foreground = fg;
        self.background = bg;
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}
