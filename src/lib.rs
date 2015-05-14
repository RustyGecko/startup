#![no_std]
#![crate_type="lib"]
#![crate_name="startup"]
#![feature(lang_items, no_std, core, start)]

pub static NO_DEAD_CODE: u32 = 0;

#[macro_use]
extern crate core;
extern crate libc;

pub mod lang_items;
pub mod mem;

#[start]
#[lang = "start"]
fn start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {

    use core::intrinsics::transmute;

    unsafe {
        let main: fn() = transmute(main);
        main();
    };

    0
}
