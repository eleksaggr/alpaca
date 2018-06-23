#![feature(abi_x86_interrupt, asm, const_fn, extern_prelude, panic_implementation)]
#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate spin;

#[macro_use]
mod util;

#[macro_use]
mod io;
mod interrupt;
pub mod panic;

use io::vga::{Color, WRITER};

#[no_mangle]
pub extern "C" fn main() {
    banner();

    logln!("Interrupts are being initialized...");
    interrupt::init();

    logln!("Calling a breakpoint exception now.");
    unsafe {
        asm!("int 3": : : : "intel");
    }

    WRITER.lock().set_color(Color::Yellow, Color::Red);
    print!("Execution of the kernel has been halted.");

    panic!("Reached end of execution.");
}

fn banner() {
    WRITER.lock().set_color(Color::White, Color::Cyan);
    WRITER.lock().clear();
    println!("================================================================================");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                ==============                                *");
    println!("*                                |            |                                *");
    println!("*                                | Aku - v0.1 |                                *");
    println!("*                                |            |                                *");
    println!("*                                ==============                                *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    println!("*                                                                              *");
    print!("================================================================================");
}
