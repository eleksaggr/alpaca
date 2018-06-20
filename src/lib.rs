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

#[no_mangle]
pub extern "C" fn main() {
    banner();

    logln!("Interrupts are being initialized...");
    interrupt::init();

    logln!("Calling a breakpoint exception now.");
    unsafe {
        asm!("int 3": : : : "intel");
    }

    logln!("Triggering a page fault, that will trigger a double fault.");
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    panic!("Reached end of execution.");
}

fn banner() {
    use io::vga::{Color, WRITER};
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
