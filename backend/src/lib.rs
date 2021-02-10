pub mod parser;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod c_api {
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use crate::parser;

    pub(super) fn parse_file(name: *const c_char) -> Result<*const c_char, Box<dyn std::error::Error>> {
        let name = to_rust_string(name)?;

        let result = CString::new(parser::parse_file(&name)?)?;
        let p = result.as_ptr();
        std::mem::forget(result);

        Ok(p)
    }

    pub(super) fn to_rust_string(s: *const c_char) -> Result<String, Box<dyn std::error::Error>> {
        assert!(!s.is_null());
        let s = unsafe { CStr::from_ptr(s) };
        let s = s.to_str()?;

        Ok(String::from(s))
    }

    pub(super) fn write_savegame(
        original_file: *const c_char,
        modified_json: *const c_char,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let original_file = to_rust_string(original_file)?;
        let modified_json = to_rust_string(modified_json)?;

        parser::write_savegame(&original_file, &modified_json)
    }
}

/// This function will parse a savegame and it will return a JSON with the format. You must free the
/// string manually in C# after calling this function.
/// # Safety
/// If you don't free the string it will be a dangling pointer.
#[no_mangle]
pub unsafe extern "C" fn parse_file(name: *const c_char) -> *const c_char {
    match c_api::parse_file(name) {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("ERR: {}", e);
            let result = CString::new(msg).expect("Couldn't format error");
            let p = result.as_ptr();
            std::mem::forget(result);

            p
        }
    }
}

/// This is the function you must call after calling `parse_file` in order to free the string.
/// # Safety
/// If you call it on a wrong pointer it could cause a crash.
#[no_mangle]
pub unsafe extern "C" fn free_rust_string(s: *mut c_char) {
    if s.is_null() {
        println!("Pointer was null");
        return;
    }
    println!("Freeing pointer");
    CString::from_raw(s);
}

#[no_mangle]
pub extern "C" fn write_savegame(
    original_file: *const c_char,
    modified_json: *const c_char,
) -> u32 {
    match c_api::write_savegame(original_file, modified_json) {
        Ok(_) => 0,
        Err(e) => {
            println!("Error:\n{}", e);
            1
        }
    }
}
