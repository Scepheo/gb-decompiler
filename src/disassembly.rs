use data::*;
use gb::*;
use instruction_walker;
use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;
use Cartridge;

pub fn collect_instructions(cart: &Cartridge, data: &Data, address: usize) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    instruction_walker::walk(cart, data, address, (), |(), address, op_code| {
        instructions.push(Instruction { address, op_code })
    });
    instructions.sort_by_key(|instruction| instruction.address);
    instructions
}

pub fn chunk(cart: &Cartridge, data: &Data, address: usize) -> ChunkSet {
    let instructions = collect_instructions(cart, data, address);

    let jump_targets: HashSet<usize> = instructions
        .iter()
        .filter_map(|instruction| instruction.jump_target())
        .collect();

    // Build an index of addresses mapped to an instruction range
    let mut blocks = HashMap::new();
    let mut start = 0;
    let mut len = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        if jump_targets.contains(&instruction.address) && len > 0 {
            blocks.insert(instructions[start].address, (start, len));
            start = index;
            len = 0;
        }

        len += 1;

        if is_dead_end(data, instruction) || instruction.is_jump() {
            blocks.insert(instructions[start].address, (start, len));
            start = index;
            len = 0;
        }
    }

    if len > 0 {
        blocks.insert(instructions[start].address, (start, len));
    }

    // Build the actual chunk set
    let mut chunks = Vec::new();
    let mut address_map = HashMap::new();

    // Put all chunks in the set
    for (address, (start, len)) in blocks.into_iter() {
        let last_instruction = instructions[start + len - 1];

        let chunk_type = if is_dead_end(data, &last_instruction) {
            InternalChunkType::End
        } else if let (true, Some(condition)) =
            (last_instruction.is_jump(), last_instruction.condition())
        {
            InternalChunkType::Conditional {
                condition,
                if_true: 0,
                if_false: 0,
            }
        } else {
            InternalChunkType::Unconditional { next: 0 }
        };

        let index = chunks.len();
        chunks.push(InternalChunk {
            chunk_type,
            start,
            len,
        });
        address_map.insert(address, index);
    }

    // Link the chunks
    for chunk in chunks.iter_mut() {
        let instruction = instructions[chunk.start + chunk.len - 1];
        let next_address = instruction.address + instruction.size();

        match &mut chunk.chunk_type {
            InternalChunkType::End => (),
            InternalChunkType::Conditional {
                if_true, if_false, ..
            } => {
                let jump_address = instruction.jump_target().unwrap();
                *if_true = *address_map.get(&jump_address).unwrap();
                *if_false = *address_map.get(&next_address).unwrap();
            }
            InternalChunkType::Unconditional { next } => {
                let address = instruction.jump_target().unwrap_or(next_address);
                *next = *address_map.get(&address).unwrap();
            }
        }
    }

    ChunkSet {
        instructions,
        chunks,
    }
}

fn is_dead_end(data: &Data, instruction: &Instruction) -> bool {
    if instruction.is_conditional() {
        false
    } else if instruction.is_return() {
        true
    } else if let Some(call_target) = instruction.call_target() {
        if data
            .functions
            .get(call_target)
            .map(|function| function.can_return)
            .unwrap_or(false)
        {
            false
        } else {
            true
        }
    } else {
        false
    }
}

pub struct ChunkSet {
    chunks: Vec<InternalChunk>,
    instructions: Vec<Instruction>,
}

impl ChunkSet {
    fn get_by_index(&self, index: usize) -> Chunk {
        Chunk::from_internal(self, &self.chunks[index])
    }

    pub fn root(&self) -> Chunk {
        self.get_by_index(0)
    }
}

struct ChunkSetIter<'a> {
    set: &'a ChunkSet,
    iter: slice::Iter<'a, InternalChunk>,
}

impl<'a> Iterator for ChunkSetIter<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|internal_chunk| Chunk::from_internal(self.set, internal_chunk))
    }
}

#[derive(Clone, Copy)]
enum InternalChunkType {
    End,
    Conditional {
        condition: Condition,
        if_true: usize,
        if_false: usize,
    },
    Unconditional {
        next: usize,
    },
}

struct InternalChunk {
    chunk_type: InternalChunkType,
    start: usize,
    len: usize,
}

pub struct Chunk<'a> {
    set: &'a ChunkSet,
    internal_chunk_type: InternalChunkType,
    instructions: &'a [Instruction],
}

impl<'a> Chunk<'a> {
    fn from_internal(set: &'a ChunkSet, chunk: &'a InternalChunk) -> Self {
        Self {
            set,
            internal_chunk_type: chunk.chunk_type,
            instructions: &set.instructions[chunk.start..chunk.start + chunk.len],
        }
    }

    pub fn chunk_type(&self) -> ChunkType<'a> {
        match self.internal_chunk_type {
            InternalChunkType::End => ChunkType::End,
            InternalChunkType::Conditional {
                condition,
                if_true,
                if_false,
            } => ChunkType::Conditional {
                condition,
                if_true: self.set.get_by_index(if_true),
                if_false: self.set.get_by_index(if_false),
            },
            InternalChunkType::Unconditional { next } => ChunkType::Unconditional {
                next: self.set.get_by_index(next),
            },
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        self.instructions
    }
}

pub enum ChunkType<'a> {
    End,
    Conditional {
        condition: Condition,
        if_true: Chunk<'a>,
        if_false: Chunk<'a>,
    },
    Unconditional {
        next: Chunk<'a>,
    },
}
