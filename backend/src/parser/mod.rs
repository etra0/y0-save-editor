use deku::prelude::*;
use serde::Serialize;

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
#[deku(magic = b"YZFH")]
pub struct SaveFile {
    #[deku(pad_bytes_before = "0x24")]
    pub time: SaveTime,

    #[deku(pad_bytes_before = "0x40F")]
    pub difficulty: Difficulty,

    #[deku(pad_bytes_before = "0x86")]
    pub current_char: CurrentCharacter,

    // TODO: Write an appropiate parser for the character skills
    #[deku(pad_bytes_before = "0x1A9F")]
    pub skills_brawler: u64,
    #[deku(pad_bytes_before = "0x18")]
    pub skills_rush: u64,
    #[deku(pad_bytes_before = "0x18")]
    pub skills_beast: u64,
    #[deku(pad_bytes_before = "0x18")]
    pub skills_thug: u64,
    #[deku(pad_bytes_before = "0x18")]
    pub skills_slugger: u64,
    #[deku(pad_bytes_before = "0x18")]
    pub skills_breaker: u64,

    #[deku(pad_bytes_before = "0x58", bits = "2")]
    pub _unused: u8,

    #[deku(bits = "1")]
    pub style_mdos: bool,

    #[deku(bits = "1")]
    pub style_breaker: bool,

    #[deku(bits = "1")]
    pub style_slugger: bool,

    #[deku(bits = "1")]
    pub style_dod: bool,

    #[deku(bits = "1")]
    pub style_beast: bool,

    #[deku(bits = "1")]
    pub style_rush: bool,

    #[deku(pad_bytes_before = "0xd3f")]
    pub outfit: Outfit,

    #[deku(pad_bytes_before = "0x4106")]
    pub map_id: u8,

    #[deku(pad_bytes_before = "0xE3")]
    pub coord_x: f32,
    pub coord_y: f32,
    pub coord_z: f32,

    #[deku(pad_bytes_before = "0xB08", count = "20")]
    pub kiryu_items: Vec<Item>,

    #[deku(count = "20")]
    pub majima_items: Vec<Item>,

    #[deku(pad_bytes_before = "0x20", count = "20")]
    pub kiryu_valuables: Vec<Item>,

    #[deku(pad_bytes_before = "0x51C0", count = "15")]
    pub kiryu_weapons: Vec<Item>,

    #[deku(pad_bytes_before = "0x2184")]
    pub kiryu_money: u64,
    pub majima_money: u64,

    #[deku(pad_bytes_before = "0x110")]
    pub kiryu_cp: u32,

    #[deku(pad_bytes_before = "0x4")]
    pub majima_cp: u32,

    #[deku(pad_bytes_before = "0x1D34")]
    pub kamurocho_pool: u64,

    #[deku(pad_bytes_before = "0x28")]
    pub sotenbori_pool: u64,

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

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
pub struct Outfit {
    #[deku(bits = "1")]
    pub dod: bool,
    #[deku(bits = "1")]
    pub hanya: bool,
    #[deku(bits = "1")]
    pub dragon: bool,
    #[deku(bits = "1")]
    pub mad_dog: bool,
    #[deku(bits = "1")]
    pub judgement: bool,
    #[deku(bits = "1")]
    pub cinderella: bool,
    #[deku(bits = "1")]
    pub producer: bool,
    #[deku(bits = "1")]
    pub parka: bool,
    #[deku(bits = "1")]
    pub lotn: bool,
    #[deku(bits = "1")]
    pub new_hire: bool,

    #[deku(bits = "6")]
    _unused: u8
}

#[derive(DekuRead, DekuWrite, Debug, Serialize)]
pub struct Item {
    id: u16,
    #[deku(pad_bytes_before = "0x4", pad_bytes_after = "0x8")]
    uses: u16
}
