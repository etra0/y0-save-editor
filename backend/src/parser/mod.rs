use deku::prelude::*;
use serde::Serialize;

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
#[deku(magic = b"YZFH")]
pub struct SaveFile {
    #[deku(pad_bytes_before = "0x24")]
    time: SaveTime,

    #[deku(pad_bytes_before = "0x40F")]
    difficulty: Difficulty,

    #[deku(pad_bytes_before = "0x86")]
    current_char: CurrentCharacter,

    #[deku(pad_bytes_before = "0x1B9F", bits = "2")]
    _unused: u8,

    #[deku(bits = "1")]
    style_mdos: bool,

    #[deku(bits = "1")]
    style_breaker: bool,

    #[deku(bits = "1")]
    style_slugger: bool,

    #[deku(bits = "1")]
    style_dod: bool,

    #[deku(bits = "1")]
    style_beast: bool,

    #[deku(bits = "1")]
    style_rush: bool,
}

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
pub struct SaveTime {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milli: u16
}

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
#[deku(type = "u8")]
pub enum Difficulty {
    Easy = 0x0,
    Normal,
    Hard,
    Extreme
}

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
#[deku(type = "u8")]
pub enum CurrentCharacter {
    Kiryu = 0x1,
    Majima
}
