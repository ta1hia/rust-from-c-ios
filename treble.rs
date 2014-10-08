// treble.rs - a rust library for tripling numbers

#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]

#[lang="sized"]
trait Sized {}

#[no_mangle]
pub extern fn triple(value: i32) -> i32 {
    value * 3
}
