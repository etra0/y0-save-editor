pub mod definitions;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::path::Path;
use deku::prelude::*;

#[no_mangle]
pub extern "system" fn parse_file(name: *const c_char) -> *const c_char {
    assert!(!name.is_null());
    let received = unsafe { CStr::from_ptr(name) };
    let path_string = {
        let s = received.to_str().unwrap();
        String::from(s)
    };

    let path = Path::new(&path_string);
    if !path.is_file() {
        println!("path wasn't a file");
        return std::ptr::null();
    }

    let file = std::fs::read(path).unwrap();

    let (_rest, save) = definitions::SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    let serialized = serde_json::to_string(&save).unwrap();

    let result = CString::new(serialized).unwrap();
    let p = result.as_ptr();
    std::mem::forget(result);

    p
}

#[no_mangle]
pub extern "system" fn write_savegame(original_file: *const c_char, modified_json: *const c_char) -> u32 {
    let original_file = to_rust_string(original_file);
    let modified_json = to_rust_string(modified_json);

    let path = Path::new(&original_file);
    if !path.is_file() {
        println!("path wasn't a file");
        return 1;
    }
    
    let mut file = std::fs::read(path).unwrap();
    let (_rest, save) = definitions::SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    let modified_save: definitions::SaveFile = serde_json::from_str(&modified_json)
        .expect("Couldn't parse JSON savegame");

    let modified_save_bytes = modified_save.to_bytes().unwrap();
    let save_bytes = save.to_bytes().unwrap();

    let diff = get_modified_bytes(&save_bytes, &modified_save_bytes);

    for off in diff {
        println!("Modifying {:x}", off);
        file[off] = modified_save_bytes[off];
    }

    std::fs::write("output.sav", file).expect("Couldn't write the file");
    println!("SaveFile created correctly!");

    0
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

fn to_rust_string(s: *const c_char) -> String {
    assert!(!s.is_null());
    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().expect("Couldn't parse String from Rust");
    String::from(s)
}

// This function should receive both the original json and the new json
pub fn get_modified_bytes(original: &Vec<u8>, new: &Vec<u8>) -> Vec<usize> {
    let mut offsets: Vec<usize> = vec![];
    original
        .iter()
        .zip(new.iter())
        .enumerate()
        .for_each(|(i, (a, b))| if a != b { offsets.push(i) });

    offsets
}
