use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;

use instructions::Instruction;
use instructions::DecodeError;

#[derive(Eq, PartialEq, Hash)]
pub struct Todo {
    start_address: usize,
    return_addresses: Vec<usize>,
}

impl Todo {
    fn new(start_address: usize) -> Todo {
        let return_addresses = Vec::new();

        Todo {
            start_address,
            return_addresses,
        }
    }

    fn continue_from(&self, start_address: usize) -> Todo {
        let return_addresses = self.return_addresses.clone();

        Todo {
            start_address,
            return_addresses,
        }
    }

    fn call(&self, start_address: usize, return_address: usize) -> Todo {
        let mut return_addresses = self.return_addresses.clone();
        return_addresses.push(return_address);

        Todo {
            start_address,
            return_addresses,
        }
    }

    fn ret(&self) -> Todo {
        let mut return_addresses = self.return_addresses.clone();
        let start_address = return_addresses.pop().unwrap();

        Todo {
            start_address,
            return_addresses,
        }
    }

    fn has_return(&self) -> bool {
        !self.return_addresses.is_empty()
    }
}

pub struct AnalysisData {
    pub todo: Vec<Todo>,
    pub done: HashSet<Todo>,
    pub unknown_jumps: Vec<usize>,
    pub ancestors: HashMap<usize, Vec<usize>>,
}

impl AnalysisData {
    fn new() -> AnalysisData {
        AnalysisData {
            todo: Vec::new(),
            done: HashSet::new(),
            unknown_jumps: Vec::new(),
            ancestors: HashMap::new(),
        }
    }

    fn add_ancestor(&mut self, from: usize, to: usize) {
        self.ancestors.entry(to).or_insert(Vec::new()).push(from);
    }

    fn get_ancestors(&self, address: &usize) -> slice::Iter<usize> {
        self.ancestors
            .get(address)
            .map(|a| a.iter())
            .unwrap_or([].iter())
    }
}

pub fn analyse(rom: &Box<[u8]>) -> Result<AnalysisData, DecodeError> {
    let mut data = AnalysisData::new();

    match analyse_static_paths(rom, &mut data) {
        Ok(()) => Ok(data),
        Err(error) => {
            print_error(&rom, &data, error);
            Err(error)
        }
    }
}

pub fn print_error(rom: &Box<[u8]>, data: &AnalysisData, error: DecodeError) {
    println!("");
    println!("Error trace:");
    println!("---------------------------------------------------------------");

    let mut stack = Vec::new();

    {
        let mut current_address = error.address;

        while current_address != 0x0100 {
            let mut ancestors = data.get_ancestors(&current_address);
            current_address = *ancestors.next().unwrap();
            stack.push(current_address);
        }
    }

    loop {
        let address: usize;

        match stack.pop() {
            Some(add) => address = add,
            None => break,
        };

        let opcode = Instruction::decode_at(&rom, address).unwrap();
        println!("{0:04X}: {1}", address, opcode);
    }

    println!("{0:04X}: {1:02X}", error.address, error.opcode);
}

fn analyse_static_paths(rom: &Box<[u8]>, data: &mut AnalysisData) -> Result<(), DecodeError> {
    // Push the rom entry point
    let entry_point = Todo::new(0x0100);
    data.todo.push(entry_point);

    loop {
        match data.todo.pop() {
            None => return Ok(()),
            Some(todo) => {
                if data.done.contains(&todo) {
                    continue;
                }

                let mut next_todo_list = try!(analyse_path(rom, data, &todo));
                data.todo.append(&mut next_todo_list);

                data.done.insert(todo);
            }
        }
    }
}

fn get_rst_value(instruction: Instruction) -> usize {
    match instruction {
        Instruction::RST_00H => 0x00,
        Instruction::RST_08H => 0x08,
        Instruction::RST_10H => 0x10,
        Instruction::RST_18H => 0x18,
        Instruction::RST_20H => 0x20,
        Instruction::RST_28H => 0x28,
        Instruction::RST_30H => 0x30,
        Instruction::RST_38H => 0x38,
        _ => panic!("Not a RST instruction"),
    }
}

fn analyse_path(rom: &Box<[u8]>, data: &mut AnalysisData, todo: &Todo) -> Result<Vec<Todo>, DecodeError> {
    let mut next_address = todo.start_address;
    let mut result = Vec::new();

    loop {
        let current_address = next_address;
        let instruction = try!(Instruction::decode_at(&rom, current_address));
        for _ in 0..todo.return_addresses.len() {
            print!("  ");
        }
        println!("{0:04X}: {1}", current_address, instruction);
        next_address += instruction.size();

        match instruction {
            Instruction::JP_a16(value) => {
                let target = value.value as usize;

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            Instruction::JP_C_a16(value)
            | Instruction::JP_NC_a16(value)
            | Instruction::JP_Z_a16(value)
            | Instruction::JP_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::JP_pHL => {
                data.unknown_jumps.push(current_address);
                return Ok(result);
            }
            Instruction::JR_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            Instruction::JR_C_r8(value)
            | Instruction::JR_NC_r8(value)
            | Instruction::JR_Z_r8(value)
            | Instruction::JR_NZ_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::CALL_C_a16(value)
            | Instruction::CALL_NC_a16(value)
            | Instruction::CALL_Z_a16(value)
            | Instruction::CALL_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(todo.call(target, next_address));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::CALL_a16(value) => {
                let target = value.value as usize;

                result.push(todo.call(target, next_address));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            Instruction::RET => {
                if !todo.has_return() {
                    panic!("No return address while at return instruction");
                }

                let next_todo = todo.ret();
                let return_address = next_todo.start_address;

                result.push(next_todo);
                data.add_ancestor(current_address, return_address);

                return Ok(result);
            }
            Instruction::RET_C
            | Instruction::RET_NC
            | Instruction::RET_Z
            | Instruction::RET_NZ
            | Instruction::RETI => {
                if !todo.has_return() {
                    panic!("No return address while at return instruction");
                }

                let next_todo = todo.ret();
                let return_address = next_todo.start_address;

                result.push(next_todo);
                data.add_ancestor(current_address, return_address);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_00H
            | Instruction::RST_08H
            | Instruction::RST_10H
            | Instruction::RST_18H
            | Instruction::RST_20H
            | Instruction::RST_28H
            | Instruction::RST_30H
            | Instruction::RST_38H => {
                let value = get_rst_value(instruction);

                result.push(todo.call(value, next_address));
                data.add_ancestor(current_address, value);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            _ => {
                data.add_ancestor(current_address, next_address);
            }
        }
    }
}
