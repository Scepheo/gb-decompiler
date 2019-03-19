mod analyzer;
mod instructions;

use std::env;
use std::fs::File;
use std::io::prelude::*;

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

    match analyzer::analyse(&rom) {
        Ok(data) => {
            println!("ROM Analysis successful");
            log_data(&data);
        },
        Err(msg) => println!("Error reading ROM: {}", msg),
    }
}

fn log_data(data: &analyzer::AnalysisData) -> () {
    if !data.unknown_jumps.is_empty() {
        println!("Unknown jumps:");

        for unknown_jump in &data.unknown_jumps {
            println!("    {:X}", unknown_jump);
        }
    }
}
