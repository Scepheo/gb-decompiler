use std::collections::HashSet;

use instructions::Instruction;

struct Jump {
    from: usize,
    to: usize
}

pub struct Todo {
    start_address: usize,
    return_address: Option<usize>
}

impl Todo {
    fn new(start_address: usize) -> Todo {
        Todo {
            start_address: start_address,
            return_address: Option::None
        }
    }

    fn new_return(start_address: usize, return_address: usize) -> Todo {
        Todo {
            start_address: start_address,
            return_address: Option::Some(return_address)
        }
    }
}

pub struct AnalysisData {
    todo: Vec<Todo>,
    done: HashSet<usize>,
    jumps: Vec<Jump>,
    unknown_jumps: Vec<usize>
}

pub fn analyse(rom: &Box<[u8]>) -> AnalysisData {
    let mut data = AnalysisData {
        todo: Vec::new(),
        done: HashSet::new(),
        jumps: Vec::new(),
        unknown_jumps: Vec::new()
    };

    analyse_static_paths(rom, &mut data);

    data
}

fn analyse_static_paths(rom: &Box<[u8]>, data: &mut AnalysisData) {
    // Push the rom entry point
    data.todo.push(Todo::new(0x0100));
    data.done.insert(0x0100);

    loop {
        match data.todo.pop() {
            None => break,
            Some(start) => {
                let next_todos = analyse_path(rom, data, start);
                
                for next_todo in next_todos {
                    let next_start = next_todo.start_address;

                    if !data.done.contains(&next_start) {
                        data.todo.push(next_todo);
                        data.done.insert(next_start);
                    }
                }
            }
        }
    }
}

fn analyse_path(rom: &Box<[u8]>, data: &mut AnalysisData, todo: Todo) -> Vec<Todo> {
    let mut next_address = todo.start_address;
    let mut result = Vec::new();

    loop {
        let current_address = next_address;
        let instruction = Instruction::decode_at(&rom, current_address);
        next_address += instruction.size();

        match instruction {
            Instruction::JP_a16(value) => {
                let target = value.value as usize;

                result.push(Todo::new(target));
                data.jumps.push(Jump { from: current_address, to: target });

                return result;
            },
            Instruction::JP_C_a16(value) | Instruction::JP_NC_a16(value) | Instruction::JP_Z_a16(value) | Instruction::JP_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(Todo::new(target));
                data.jumps.push(Jump { from: current_address, to: target });

                result.push(Todo::new(next_address));
                data.jumps.push(Jump { from: current_address, to: next_address });

                return result;
            },
            Instruction::JP_pHL => {
                data.unknown_jumps.push(current_address);
                return result;
            },
            Instruction::JR_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(Todo::new(target));
                data.jumps.push(Jump { from: current_address, to: target });

                return result;
            },
            Instruction::JR_C_r8(value) | Instruction::JR_NC_r8(value) | Instruction::JR_Z_r8(value) | Instruction::JR_NZ_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(Todo::new(target));
                data.jumps.push(Jump { from: current_address, to: target });

                result.push(Todo::new(next_address));
                data.jumps.push(Jump { from: current_address, to: next_address });

                return result;
            },
            Instruction::CALL_a16(value) => {
                let target = value.value as usize;

                result.push(Todo::new_return(target, next_address));
                data.jumps.push(Jump { from: current_address, to: target });

                return result;
            },
            Instruction::CALL_C_a16(value) | Instruction::CALL_NC_a16(value) | Instruction::CALL_Z_a16(value) | Instruction::CALL_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(Todo::new_return(target, next_address));
                data.jumps.push(Jump { from: current_address, to: target });

                result.push(Todo::new(next_address));
                data.jumps.push(Jump { from: current_address, to: next_address });

                return result;
            },
            Instruction::RET => {
                let return_address = todo.return_address.expect("No return address found");

                result.push(Todo::new(return_address));
                data.jumps.push(Jump { from: current_address, to: return_address });

                return result;
            },
            Instruction::RET_C | Instruction::RET_NC | Instruction::RET_Z | Instruction::RET_NZ => {
                let return_address = todo.return_address.expect("No return address found");

                result.push(Todo::new(return_address));
                data.jumps.push(Jump { from: current_address, to: return_address });

                result.push(Todo::new_return(next_address, return_address));
                data.jumps.push(Jump { from: current_address, to: next_address });

                return result;
            },
            _ => continue
        }
    }
}
