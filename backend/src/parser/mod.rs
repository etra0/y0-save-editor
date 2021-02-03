pub mod definitions;

use deku::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

pub(crate) fn parse_file(name: *const c_char) -> Result<*const c_char, Box<dyn std::error::Error>> {
    let name = to_rust_string(name)?;

    let file = try_open_file(&name)?;

    let (_rest, save) = definitions::SaveFile::from_bytes((file.as_ref(), 0))?;

    let serialized = serde_json::to_string(&save)?;

    let result = CString::new(serialized)?;
    let p = result.as_ptr();
    std::mem::forget(result);

    Ok(p)
}

fn try_open_file(f: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = Path::new(f);
    if !path.is_file() {
        return Err(format!("The file {:?} doesn't exists", path).into());
    }

    std::fs::read(path).map_err(|_| "Couldn't read the file".into())
}

pub(crate) fn to_rust_string(s: *const c_char) -> Result<String, Box<dyn std::error::Error>> {
    assert!(!s.is_null());
    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str()?;

    Ok(String::from(s))
}

pub(crate) fn write_savegame(
    original_file: *const c_char,
    modified_json: *const c_char,
) -> Result<(), Box<dyn std::error::Error>> {
    let original_file = to_rust_string(original_file)?;
    let modified_json = to_rust_string(modified_json)?;

    let mut file_bytes = try_open_file(&original_file)?;
    let (_, save) = definitions::SaveFile::from_bytes((file_bytes.as_ref(), 0))?;
    let modified_save: definitions::SaveFile = serde_json::from_str(&modified_json)?;

    let modified_save_bytes = modified_save.to_bytes()?;
    let save_bytes = save.to_bytes()?;

    let diff = get_modified_bytes(&save_bytes, &modified_save_bytes);

    for off in diff {
        println!("Modifying {:x}", off);
        file_bytes[off] = modified_save_bytes[off];
    }

    let new_file_name = original_file + ".new";
    std::fs::write(new_file_name, file_bytes)?;

    Ok(())
}

// This function should receive both the original json and the new json
pub fn get_modified_bytes(original: &[u8], new: &[u8]) -> Vec<usize> {
    let mut offsets: Vec<usize> = vec![];
    original
        .iter()
        .zip(new.iter())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if a != b {
                offsets.push(i)
            }
        });

    offsets
}
