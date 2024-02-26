#![no_std]
#![no_main]
extern crate alloc;

use alloc::boxed::Box;
use core::ops::Deref;
use cortex_m_rt::entry;
use emballoc::Allocator;
use embedded_dma::{ReadBuffer, ReadTarget};
pub use panic_abort;

#[global_allocator]
static GLOBAL: Allocator<4096> = Allocator::new();

pub struct MyArray([u8; 100]);

impl Deref for MyArray {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

// unsafe impl ReadTarget for MyArray {
//     type Word = u8;

//     fn as_read_buffer(&self) -> (*const Self::Word, usize) {
//         self.deref().as_read_buffer()
//     }
// }

#[entry]
fn main() -> ! {
    let a = Box::new(MyArray([0; 100]));
    unsafe {
        a.read_buffer();
    }

    loop {}
}
