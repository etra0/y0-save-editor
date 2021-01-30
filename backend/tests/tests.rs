use deku::prelude::*;
use backend::parser;
use std::fs::read;
use serde::{Serialize, Deserialize};

#[test]
fn test_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = read("./tests/save_test.sav")?;
    let (_rest, mut save) = parser::SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    // Test serialization
    let serialized = serde_json::to_string(&save)?;
    println!("{}", serialized);

    let data = save.to_bytes()?;

    std::fs::write("test.bin", data)?;

    dbg!(save);

    Ok(())
}
