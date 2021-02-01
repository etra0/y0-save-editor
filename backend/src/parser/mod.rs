pub mod definitions;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::path::Path;
use deku::prelude::*;
use serde::{Serialize, Deserialize};

#[no_mangle]
pub extern "system" fn parse_file(name: *const c_char) -> *const c_char {
    assert!(!name.is_null());
    let received = unsafe { CStr::from_ptr(name) };
    let mut path_string = {
        let s = received.to_str().unwrap();
        String::from(s)
    };

    let path = Path::new(&path_string);
    if !path.is_file() {
        println!("path wasn't a file");
        return std::ptr::null();
    }

    let mut file = std::fs::read(path).unwrap();

    let (_rest, mut save) = definitions::SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    let serialized = serde_json::to_string(&save).unwrap();

    // Convert it to CString at the end
    let result = CString::new(serialized).unwrap();
    let p = result.as_ptr();
    std::mem::forget(result);
    p
}

#[no_mangle]
pub unsafe extern "system" fn free_string(s: *mut c_char) {
    if s.is_null() {
        println!("Pointer was null");
        return;
    }
    println!("Freeing pointer");
    CString::from_raw(s);
}
