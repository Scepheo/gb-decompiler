mod instructions;
mod analyzer;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn get_filename() -> String {
    let mut args = env::args();
    args.nth(1).expect("Filename is required")
}

fn load_rom(filename: &str) -> Box<[u8]> {
    let mut file = File::open(&filename).expect("file not found");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("error reading file");
    data.into_boxed_slice()
}

fn main() {
    let filename = get_filename();
    let rom = load_rom(&filename);
    analyzer::analyse(&rom);
}
