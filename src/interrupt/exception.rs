#[derive(Clone, Copy, Debug)]
pub struct StackFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}

pub extern "x86-interrupt" fn breakpoint(frame: &mut StackFrame) {
    logln!("Exception: Breakpoint\n{:#x?}", frame);
}

pub extern "x86-interrupt" fn double(frame: &mut StackFrame, error: u64) {
    logln!("Exception: Double Fault\n{:#x?}\nError Code: {}", frame, error);
}
