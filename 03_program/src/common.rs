pub fn c_str_to_string(c_str: *const std::ffi::c_char) -> Option<String> {
    unsafe {
        if c_str.is_null() {
            None
        } else {
            let byte = std::ffi::CStr::from_ptr(c_str).to_bytes();
            let str = String::from_utf8_lossy(byte).into_owned();
            Some(str)
        }
    }
}
