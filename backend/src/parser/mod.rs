pub mod definitions;

use deku::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

pub fn parse_file(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = try_open_file(name)?;

    let (_rest, save) = definitions::SaveFile::from_bytes((file.as_ref(), 0))?;

    let serialized = serde_json::to_string_pretty(&save)?;

    Ok(serialized)
}

fn try_open_file(f: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = Path::new(f);
    if !path.is_file() {
        return Err(format!("The file {:?} doesn't exists", path).into());
    }

    std::fs::read(path).map_err(|_| "Couldn't read the file".into())
}


pub fn write_savegame(
    original_file:  &str,
    modified_json: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_bytes = try_open_file(original_file)?;
    let (_, save) = definitions::SaveFile::from_bytes((file_bytes.as_ref(), 0))?;
    let modified_save: definitions::SaveFile = serde_json::from_str(modified_json)?;

    let modified_save_bytes = modified_save.to_bytes()?;
    let save_bytes = save.to_bytes()?;

    let diff = get_modified_bytes(&save_bytes, &modified_save_bytes);

    for off in diff {
        println!("Modifying {:x}", off);
        file_bytes[off] = modified_save_bytes[off];
    }

    let new_file_name = original_file.to_string() + ".new";
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
