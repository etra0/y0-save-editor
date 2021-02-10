use backend::parser::definitions::*;
use backend::parser::get_modified_bytes;
use deku::prelude::*;
use std::fs::read;

#[test]
fn test_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let file = read("./tests/save_test.sav")?;
    let (_rest, save) = SaveFile::from_bytes((file.as_ref(), 0)).unwrap();

    // Test serialization
    let serialized = serde_json::to_string(&save)?;
    println!("{}", serialized);

    let data = save.to_bytes()?;

    dbg!(save);

    Ok(())
}

#[test]
fn test_modification() -> Result<(), Box<dyn std::error::Error>> {
    let file = read("./tests/save_test.sav")?;
    let (_rest, save) = SaveFile::from_bytes((file.as_ref(), 0))?;
    let (_rest, mut copy) = SaveFile::from_bytes((file.as_ref(), 0))?;

    copy.kiryu_money = u64::MAX;
    let difference = get_modified_bytes(&save.to_bytes()?, &copy.to_bytes()?);
    let range: Vec<usize> = (0xf2c0..0xf2c8).collect();
    assert_eq!(&difference, &range);

    Ok(())
}
