use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;

use instructions::Instruction;

pub struct Todo {
    start_address: usize,
    return_address: Option<usize>,
}

pub struct AnalysisData {
    todo: Vec<Todo>,
    done: HashSet<usize>,
    unknown_jumps: Vec<usize>,
    ancestors: HashMap<usize, Vec<usize>>,
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

    fn get_ancestors(&mut self, address: &usize) -> slice::Iter<usize> {
        self.ancestors
            .get(address)
            .map(|a| a.iter())
            .unwrap_or([].iter())
    }
}

pub fn analyse(rom: &Box<[u8]>) -> Result<AnalysisData, String> {
    let mut data = AnalysisData::new();
    try!(analyse_static_paths(rom, &mut data));
    Ok(data)
}

fn analyse_static_paths(rom: &Box<[u8]>, data: &mut AnalysisData) -> Result<(), String> {
    // Push the rom entry point
    data.todo.push(Todo {
        start_address: 0x0100,
        return_address: None,
    });
    data.done.insert(0x0100);

    loop {
        match data.todo.pop() {
            None => return Ok(()),
            Some(start) => {
                let next_todos = try!(analyse_path(rom, data, start));

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

fn analyse_path(rom: &Box<[u8]>, data: &mut AnalysisData, todo: Todo) -> Result<Vec<Todo>, String> {
    let mut next_address = todo.start_address;
    let mut result = Vec::new();

    loop {
        let current_address = next_address;
        let instruction = try!(Instruction::decode_at(&rom, current_address));
        next_address += instruction.size();

        match instruction {
            Instruction::JP_a16(value) => {
                let target = value.value as usize;

                result.push(Todo {
                    start_address: target,
                    ..todo
                });
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            Instruction::JP_C_a16(value)
            | Instruction::JP_NC_a16(value)
            | Instruction::JP_Z_a16(value)
            | Instruction::JP_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(Todo {
                    start_address: target,
                    ..todo
                });
                data.add_ancestor(current_address, target);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::JP_pHL => {
                data.unknown_jumps.push(current_address);
                return Ok(result);
            }
            Instruction::JR_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(Todo {
                    start_address: target,
                    ..todo
                });
                data.add_ancestor(current_address, target);

                return Ok(result);
            }
            Instruction::JR_C_r8(value)
            | Instruction::JR_NC_r8(value)
            | Instruction::JR_Z_r8(value)
            | Instruction::JR_NZ_r8(value) => {
                let target = next_address.wrapping_add(value.value as usize);

                result.push(Todo {
                    start_address: target,
                    ..todo
                });
                data.add_ancestor(current_address, target);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::CALL_a16(value)
            | Instruction::CALL_C_a16(value)
            | Instruction::CALL_NC_a16(value)
            | Instruction::CALL_Z_a16(value)
            | Instruction::CALL_NZ_a16(value) => {
                let target = value.value as usize;

                result.push(Todo {
                    start_address: target,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, target);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RET => {
                let return_address = todo.return_address.expect("No return address found");

                // Continuation already returned at call site
                data.add_ancestor(current_address, return_address);

                return Ok(result);
            }
            Instruction::RET_C
            | Instruction::RET_NC
            | Instruction::RET_Z
            | Instruction::RET_NZ
            | Instruction::RETI => {
                let return_address = todo.return_address.expect("No return address found");

                // Continuation already returned at call site
                data.add_ancestor(current_address, return_address);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_00H => {
                result.push(Todo {
                    start_address: 0x00,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x00);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_08H => {
                result.push(Todo {
                    start_address: 0x08,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x08);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_10H => {
                result.push(Todo {
                    start_address: 0x10,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x10);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_18H => {
                result.push(Todo {
                    start_address: 0x18,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x18);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_20H => {
                result.push(Todo {
                    start_address: 0x20,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x20);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_28H => {
                result.push(Todo {
                    start_address: 0x28,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x28);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_30H => {
                result.push(Todo {
                    start_address: 0x30,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x30);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            Instruction::RST_38H => {
                result.push(Todo {
                    start_address: 0x38,
                    return_address: Some(next_address),
                });
                data.add_ancestor(current_address, 0x38);

                result.push(Todo {
                    start_address: next_address,
                    ..todo
                });
                data.add_ancestor(current_address, next_address);

                return Ok(result);
            }
            _ => {
                data.add_ancestor(current_address, next_address);
            }
        }
    }
}
