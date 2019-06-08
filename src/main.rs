#![no_std]
#![no_main]

//use std::fs::File;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    //let _file = File::create("foo.txt").unwrap();
    let msg = "\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\\(^o^)/\n";
    let slice = msg.as_bytes();
    let ptr = slice as *const [u8] as *const u8;
    unsafe {
        kaku(1, ptr, 256);
        yameru(0);
    }
    
}

#[link(name="syscall", kind="static")]
extern "C" {
    fn kaku(fd: i64, ptr: *const u8, len: usize);
}
extern "C" {
    fn yameru(fd: i64);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
