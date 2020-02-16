pub mod pic;

use spin::Mutex;

lazy_static! {
    pub static ref PIC: Mutex<pic::ChainedPic> = Mutex::new({ pic::ChainedPic::new(0x20, 0xA0) });
}
