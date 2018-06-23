use super::StackFrame;

pub extern "x86-interrupt" fn breakpoint(frame: &mut StackFrame) {
    logln!("Exception: Breakpoint\n{:#x?}", frame);
}

pub extern "x86-interrupt" fn double(frame: &mut StackFrame, error: u64) {
    logln!("Exception: Double Fault\n{:#x?}\nError Code: {}", frame, error);
}
