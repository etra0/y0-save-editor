use clap::Clap;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use backend::parser::{parse_file, write_savegame};
use std::fs::write;
use serde_json;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Sebastián Aedo (@etra0)", name = "save-editor-cli")]
struct Opts {
    #[clap(about = "Path of the original savefile")]
    input_save: String,

    #[clap(short = 'j', long, about = "If you already modified the JSON, you can generate the savefile passing it as input as well")]
    input_json: Option<String>,
}

fn write_json(filename: &str, json: &str) {
    let filename = filename.to_string() + ".json";

    std::fs::write(filename, json).unwrap();
}


fn main() -> Result<(), std::io::Error> {
    let args = Opts::parse();
    let save = PathBuf::from(args.input_save);
    
    if !save.is_file() {
        let err = Error::new(ErrorKind::Other, "File doesn't exists");
        return Err(err);
    }

    let save = save.to_str().unwrap();
    match args.input_json {
        Some(f) => {
            let json = std::fs::read_to_string(f).unwrap();
            write_savegame(save, &json);
        },
        None => {
            println!("Dumping the JSON");
            let json = parse_file(save).unwrap();
            write_json(save, &json);
        }
    }

    Ok(())
}
