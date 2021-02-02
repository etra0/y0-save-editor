use deku::prelude::*;
use backend::parser::definitions::*;
use std::fs::read;
use serde::{Serialize, Deserialize};
use backend::parser::get_modified_bytes;

#[test]
fn test_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = read("./tests/save_test.sav")?;
    let (_rest, mut save) = SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    // Test serialization
    let serialized = serde_json::to_string(&save)?;
    println!("{}", serialized);

    let data = save.to_bytes()?;

    std::fs::write("test.bin", data)?;

    dbg!(save);

    Ok(())
}

#[test]
fn test_modification() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = read("./tests/save_test.sav")?;
    let (_rest, mut save) = SaveFile::from_bytes((file.as_ref(), 0))?;
    let (_rest, mut copy) = SaveFile::from_bytes((file.as_ref(), 0))?;

    copy.kiryu_money = u64::MAX;
    let difference = get_modified_bytes(&save.to_bytes()?, &copy.to_bytes()?);
    let range: Vec<usize> = (0xf2c0..0xf2c8).collect();
    assert_eq!(&difference, &range);

    Ok(())
}
