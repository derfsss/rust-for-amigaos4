/// Print a null-terminated byte string to the serial debug output.
#[macro_export]
macro_rules! debug_print {
    ($s:expr) => {
        unsafe { amigaos4_sys::amiga_debug_str($s.as_ptr()) }
    };
}

/// Print a null-terminated byte string followed by a newline to serial debug output.
#[macro_export]
macro_rules! debug_println {
    ($s:expr) => {
        unsafe {
            amigaos4_sys::amiga_debug_str($s.as_ptr());
            amigaos4_sys::amiga_debug_str(b"\n\0".as_ptr());
        }
    };
}
