#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;
static HELLO: &[u8] = b"Heeeeeelllloo!";
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("100% copy{}", "!");

    loop {}


}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
