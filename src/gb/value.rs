use gb::Cartridge;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct d8 {
    pub value: u8,
}

impl d8 {
    pub fn at(cartridge: &Cartridge, address: usize) -> d8 {
        d8 {
            value: cartridge[address],
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
    pub fn at(cartridge: &Cartridge, address: usize) -> d16 {
        d16 {
            value: u16::from_le_bytes([cartridge[address], cartridge[address + 1]]),
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
    pub fn at(cartridge: &Cartridge, address: usize) -> a8 {
        a8 {
            value: cartridge[address],
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
    pub fn at(cartridge: &Cartridge, address: usize) -> a16 {
        a16 {
            value: u16::from_le_bytes([cartridge[address], cartridge[address + 1]]),
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
    pub fn at(cartridge: &Cartridge, address: usize) -> r8 {
        r8 {
            value: cartridge[address] as i8,
        }
    }
}

impl fmt::Display for r8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value >= 0 {
            write!(f, "+{0}", self.value)
        } else {
            write!(f, "{0}", self.value)
        }
    }
}
