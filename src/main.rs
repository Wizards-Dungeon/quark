// Dear reader, you may wonder WHY THE FUCK I have used unwrap_unchecked everywhere
// if environment variables like WINDIR are unset or critical registry keys don't contain
// valid ASCII characters, please blow up your own computer yourself.
#![feature(lang_items)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::{
    ffi::CStr,
    hash::{self, Hash, Hasher},
};

#[macro_use]
pub extern crate alloc;

mod imp;
use imp::*;

mod antidebug;
mod editions;
mod v_strings;

use winapi::um::{processthreadsapi::ExitProcess, synchapi::Sleep};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub unsafe extern "C" fn _start(_: isize, _: *const *const u8) -> isize {
    libc::printf(v_strings::v_credits()());

    let windir = CStr::from_ptr(libc::getenv(c!("WINDIR")));
    let slmgr = cformat!(
        "{}\\System32\\slmgr.vbs",
        windir.to_str().unwrap_unchecked()
    );

    set_pk(&slmgr);
    create_ticket();
    install_license(windir, &slmgr);
    show_activation(&slmgr);

    Sleep(0xFFFFFFFF);
    core::hint::unreachable_unchecked();
}

#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    ExitProcess(1);
    core::hint::unreachable_unchecked();
}
