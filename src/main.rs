#![windows_subsystem = "windows"]

use windows::{
    Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW},
    core::w,
};

fn main() {
    unsafe {
        MessageBoxW(None, w!("Hellow world!"), w!("TITLE TEXT"), MB_OK);
    }
}
