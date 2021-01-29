mod parser;

use deku::prelude::*;
use parser::*;
use std::fs::read;
use serde::{Serialize, Deserialize};

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = read("./data/SaveData0005.sav")?;
    let (_rest, save) = SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    let serialized = serde_json::to_string(&save).unwrap();

    let data = save.to_bytes().unwrap();

    println!("{}", serialized);

    std::fs::write("test.bin", data)?;

    dbg!(save);

    Ok(())
}
