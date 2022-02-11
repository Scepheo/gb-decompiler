use data::*;
use gb::*;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn walk<T, F>(cartridge: &Cartridge, data: &Data, address: usize, initial: T, mut func: F)
where
    T: Copy,
    F: FnMut(T, usize, OpCode) -> T,
{
    let mut todo: VecDeque<(usize, T)> = VecDeque::new();
    let mut enqueued: HashSet<usize> = HashSet::new();

    macro_rules! add_todo {
        ($address:expr, $value:expr) => {
            if enqueued.insert($address) {
                todo.push_back(($address, $value));
            }
        };
    }

    add_todo!(address, initial);

    while let Some((address, input)) = todo.pop_front() {
        let instruction = OpCode::decode_at(cartridge, address).unwrap();
        let next_address = address + instruction.size();

        let output = func(input, address, instruction);

        if instruction.can_continue()
            || instruction
                .call_target(address)
                .and_then(|call_address| data.functions.get(call_address))
                .map_or(false, |function| function.can_return)
        {
            add_todo!(next_address, output);
        }

        if let Some(jump_address) = instruction.jump_target(address) {
            add_todo!(jump_address, output);
        }
    }
}
