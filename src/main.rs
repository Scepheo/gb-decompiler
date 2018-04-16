mod instructions;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use instructions::Instruction;

fn get_filename() -> String {
    let mut args = env::args();
    args.nth(1).expect("Filename is required")
}

fn load_file(filename: &str) -> Box<[u8]> {
    let mut file = File::open(&filename).expect("file not found");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("error reading file");
    data.into_boxed_slice()
}

fn main() {
    let filename = get_filename();
    let data = load_file(&filename);

    let mut address = 0x0100;

    while address < 0x0104 {
        let instruction = Instruction::decode_at(&data, address);
        address += instruction.size();
        println!("{}", instruction);
    }
}
