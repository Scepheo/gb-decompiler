use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Data {
    pub functions: FunctionTable,
}

impl Data {
    pub fn new() -> Data {
        Data {
            functions: FunctionTable::new(),
        }
    }
}

pub struct Function {
    pub address: usize,
    pub name: String,
    pub call_sites: HashSet<usize>,
    pub can_return: bool,
}

impl Function {
    pub fn new(address: usize) -> Function {
        Function {
            address,
            name: format!("function_{}", address),
            call_sites: HashSet::new(),
            can_return: false,
        }
    }
}

pub struct FunctionTable {
    functions: HashMap<usize, Function>,
}

impl FunctionTable {
    pub fn new() -> FunctionTable {
        FunctionTable {
            functions: HashMap::new(),
        }
    }

    pub fn get_or_add(&mut self, address: usize) -> (bool, &mut Function) {
        match self.functions.entry(address) {
            Entry::Occupied(occupied) => (false, occupied.into_mut()),
            Entry::Vacant(vacant) => (true, vacant.insert(Function::new(address))),
        }
    }

    pub fn get(&self, address: usize) -> Option<&Function> {
        self.functions.get(&address)
    }

    pub fn get_mut(&mut self, address: usize) -> Option<&mut Function> {
        self.functions.get_mut(&address)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Function)> {
        self.functions.iter()
    }
}
