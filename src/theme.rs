use std::ffi::c_void;
use windows::{Win32::System::Registry::*, core::w};

pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn current() -> Self {
        let mut light = 1u32; // Windows' default
        let mut size = size_of::<u32>() as u32;

        let _ = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
                w!("AppsUseLightTheme"),
                RRF_RT_REG_DWORD,
                None,
                Some(&mut light as *mut u32 as *mut c_void),
                Some(&mut size),
            )
        };

        if light == 0 {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
