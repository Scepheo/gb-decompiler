use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct d8 {
    pub value: u8,
}

impl d8 {
    pub fn at(data: &Box<[u8]>, address: usize) -> d8 {
        d8 {
            value: data[address],
        }
    }
}

impl fmt::Display for d8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:02X}", self.value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct d16 {
    pub value: u16,
}

impl d16 {
    pub fn at(data: &Box<[u8]>, address: usize) -> d16 {
        let low = data[address] as u16;
        let high = data[address + 1] as u16;
        d16 {
            value: low | (high << 8),
        }
    }
}

impl fmt::Display for d16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:04X}", self.value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct a8 {
    pub value: u8,
}

impl a8 {
    pub fn at(data: &Box<[u8]>, address: usize) -> a8 {
        a8 {
            value: data[address],
        }
    }
}

impl fmt::Display for a8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${0:02X}", self.value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct a16 {
    pub value: u16,
}

impl a16 {
    pub fn at(data: &Box<[u8]>, address: usize) -> a16 {
        let low = data[address] as u16;
        let high = data[address + 1] as u16;
        a16 {
            value: low | (high << 8),
        }
    }
}

impl fmt::Display for a16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${0:04X}", self.value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct r8 {
    pub value: i8,
}

impl r8 {
    pub fn at(data: &Box<[u8]>, address: usize) -> r8 {
        r8 {
            value: data[address] as i8,
        }
    }
}

impl fmt::Display for r8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:+02X}", self.value)
    }
}
