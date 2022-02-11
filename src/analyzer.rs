use data::Data;
use disassembly;
use gb::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;

pub trait RomAnalyzer {
    fn run(&self, cartridge: &Cartridge, data: &mut Data) -> bool;

    fn run_until_unchanged(&self, cartridge: &Cartridge, data: &mut Data) -> bool {
        let mut any_changes = false;

        loop {
            let changes = self.run(cartridge, data);
            any_changes |= changes;

            if !changes {
                break;
            }
        }

        any_changes
    }
}

struct FunctionAnalyzer;

impl RomAnalyzer for FunctionAnalyzer {
    fn run(&self, cart: &Cartridge, data: &mut Data) -> bool {
        let (is_new, entrypoint) = data.functions.get_or_add(Cartridge::ENTRY_POINT);

        if is_new {
            entrypoint.name = "entrypoint".to_string();
            true
        } else {
            analyze_functions(cart, data)
        }
    }
}

fn analyze_functions(cart: &Cartridge, data: &mut Data) -> bool {
    let known_function_addresses: Vec<_> =
        data.functions.iter().map(|(address, _)| *address).collect();

    let mut changes = false;

    for function_address in known_function_addresses.into_iter() {
        let instructions = disassembly::collect_instructions(cart, data, function_address);

        for instruction in instructions.into_iter() {
            if let Some(call_address) = instruction.call_target() {
                let (is_new_function, called_function) = data.functions.get_or_add(call_address);
                changes |= is_new_function;
                let is_new_callsite = called_function.call_sites.insert(instruction.address);
                changes |= is_new_callsite;
            }

            if instruction.is_return() {
                let function = data.functions.get_mut(function_address).unwrap();
                if !function.can_return {
                    function.can_return = true;
                    changes = true;
                }
            }
        }
    }

    changes
}

struct CompositeAnalyzer {
    inner: Vec<Box<dyn RomAnalyzer>>,
}

impl CompositeAnalyzer {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push<T: Into<Box<dyn RomAnalyzer>>>(&mut self, analyzer: T) {
        self.inner.push(analyzer.into())
    }
}

impl RomAnalyzer for CompositeAnalyzer {
    fn run(&self, cartridge: &Cartridge, data: &mut Data) -> bool {
        self.inner.iter().fold(false, |changes, analyzer| {
            changes || analyzer.run_until_unchanged(cartridge, data)
        })
    }
}

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

pub fn analyse(cartridge: &Cartridge) -> Result<AnalysisData, DecodeError> {
    let mut data = AnalysisData::new();

    match analyse_static_paths(cartridge, &mut data) {
        Ok(()) => Ok(data),
        Err(error) => {
            print_error(&cartridge, &data, error);
            Err(error)
        }
    }
}

pub fn print_error(cartridge: &Cartridge, data: &AnalysisData, error: DecodeError) {
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

        let opcode = OpCode::decode_at(&cartridge, address).unwrap();
        println!("{0:04X}: {1}", address, opcode);
    }

    println!("{0:04X}: {1:02X}", error.address, error.opcode);
}

fn analyse_static_paths(cartridge: &Cartridge, data: &mut AnalysisData) -> Result<(), DecodeError> {
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

                let mut next_todo_list = analyse_path(cartridge, data, &todo)?;
                data.todo.append(&mut next_todo_list);

                data.done.insert(todo);
            }
        }
    }
}

fn get_rst_value(instruction: OpCode) -> usize {
    match instruction {
        OpCode::RST_00H => 0x00,
        OpCode::RST_08H => 0x08,
        OpCode::RST_10H => 0x10,
        OpCode::RST_18H => 0x18,
        OpCode::RST_20H => 0x20,
        OpCode::RST_28H => 0x28,
        OpCode::RST_30H => 0x30,
        OpCode::RST_38H => 0x38,
        _ => panic!("Not a RST instruction"),
    }
}

fn analyse_path(
    cartridge: &Cartridge,
    data: &mut AnalysisData,
    todo: &Todo,
) -> Result<Vec<Todo>, DecodeError> {
    let mut next_address = todo.start_address;
    let mut result = Vec::new();

    loop {
        let current_address = next_address;
        let instruction = OpCode::decode_at(&cartridge, current_address)?;
        for _ in 0..todo.return_addresses.len() {
            print!("  ");
        }
        println!("{0:04X}: {1}", current_address, instruction);
        next_address += instruction.size();

        match instruction {
            OpCode::JP_a16(value) => {
                let target = value.value as usize;

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            OpCode::JP_C_a16(value)
            | OpCode::JP_NC_a16(value)
            | OpCode::JP_Z_a16(value)
            | OpCode::JP_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            OpCode::JP_pHL => {
                data.unknown_jumps.push(current_address);
                return Ok(result);
            }
            OpCode::JR_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            OpCode::JR_C_r8(value)
            | OpCode::JR_NC_r8(value)
            | OpCode::JR_Z_r8(value)
            | OpCode::JR_NZ_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(todo.continue_from(target));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            OpCode::CALL_C_a16(value)
            | OpCode::CALL_NC_a16(value)
            | OpCode::CALL_Z_a16(value)
            | OpCode::CALL_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(todo.call(target, next_address));
                data.add_ancestor(current_address, target);

                result.push(todo.continue_from(next_address));
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            OpCode::CALL_a16(value) => {
                let target = value.value as usize;

                result.push(todo.call(target, next_address));
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            OpCode::RET => {
                if !todo.has_return() {
                    panic!("No return address while at return instruction");
                }

                let next_todo = todo.ret();
                let return_address = next_todo.start_address;

                result.push(next_todo);
                data.add_ancestor(current_address, return_address);

                return Ok(result);
            }
            OpCode::RET_C | OpCode::RET_NC | OpCode::RET_Z | OpCode::RET_NZ | OpCode::RETI => {
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
            OpCode::RST_00H
            | OpCode::RST_08H
            | OpCode::RST_10H
            | OpCode::RST_18H
            | OpCode::RST_20H
            | OpCode::RST_28H
            | OpCode::RST_30H
            | OpCode::RST_38H => {
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
