#![no_std]
#![no_main]

mod lang_items;
#[macro_use]
mod console;
mod sbi;

use crate::sbi::shutdown;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() {
    print!("Hello, ");
    println!("world!");
    clear_bss();
    shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}