extern crate winapi;

use std::ffi::OsStr;
use std::ptr;
use std::os::windows::prelude::*;

use winapi::user32::MessageBox;

fn main() {
    let caption = "Rust";
    let text = "Hello, World!\r\nHow are you?";

    let caption_w = to_w_str(caption);
    let text_w = to_w_str(text);

    unsafe {
        MessageBox(ptr::null_mut(), text_w.as_ptr(), caption_w.as_ptr(), 0);
    }
}

fn to_w_str(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}
