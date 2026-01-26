use std::env;
use std::fs;

use openttd_savegame::header::SavegameHeader;

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("usage: openttd_cli <savegame>");
            std::process::exit(2);
        }
    };

    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(1);
        }
    };

    match SavegameHeader::parse(&bytes) {
        Ok(header) => {
            println!("compression: {:?}", header.compression);
            println!("version: {}", header.version);
            println!("flags: {}", header.flags);
        }
        Err(err) => {
            eprintln!("failed to parse header: {err}");
            std::process::exit(1);
        }
    }
}
