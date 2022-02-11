use std::ops;

pub struct Cartridge {
    data: Box<[u8]>,
}

impl Cartridge {
    pub const ENTRY_POINT: usize = 0x0100;
    pub const TYPE_ADDRESS: usize = 0x0147;

    pub fn new<T: Into<Box<[u8]>>>(data: T) -> Cartridge {
        Cartridge { data: data.into() }
    }

    pub fn cartridge_type(&self) -> Option<CartridgeType> {
        CartridgeType::from_byte(self.data[CARTRIDGE_TYPE_ADDRESS])
    }
}

impl<I> ops::Index<I> for Cartridge
where
    [u8]: ops::Index<I>,
{
    type Output = <[u8] as ops::Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index)
    }
}

const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum CartridgeType {
    ROM_ONLY = 0x00,

    MBC1 = 0x01,
    MBC1_RAM = 0x02,
    MBC1_RAM_BATTERY = 0x03,

    MBC2 = 0x05,
    MBC2_BATTERY = 0x06,

    ROM_RAM = 0x08,
    ROM_RAM_BATTERY = 0x09,

    MMM01 = 0x0B,
    MMM01_RAM = 0x0C,
    MMM01_RAM_BATTERY = 0x0D,

    MBC3_TIMER_BATTERY = 0x0F,
    MBC3_TIMER_RAM_BATTERY = 0x10,
    MBC3 = 0x11,
    MBC3_RAM = 0x12,
    MBC3_RAM_BATTERY = 0x13,

    MBC5 = 0x19,
    MBC5_RAM = 0x1A,
    MBC5_RAM_BATTERY = 0x1B,
    MBC5_RUMBLE = 0x1C,
    MBC5_RUMBLE_RAM = 0x1D,
    MBC5_RUMBLE_RAM_BATTERY = 0x1E,

    MBC6 = 0x20,

    MBC7_SENSOR_RUMBLE_RAM_BATTERY = 0x22,

    POCKET_CAMERA = 0xFC,
    BANDAI_TAMA5 = 0xFD,
    HuC3 = 0xFE,
    HuC1_RAM_BATTERY = 0xFF,
}

impl CartridgeType {
    pub fn from_byte(byte: u8) -> Option<CartridgeType> {
        match byte {
            0x00 => Some(CartridgeType::ROM_ONLY),
            0x01 => Some(CartridgeType::MBC1),
            0x02 => Some(CartridgeType::MBC1_RAM),
            0x03 => Some(CartridgeType::MBC1_RAM_BATTERY),
            0x05 => Some(CartridgeType::MBC2),
            0x06 => Some(CartridgeType::MBC2_BATTERY),
            0x08 => Some(CartridgeType::ROM_RAM),
            0x09 => Some(CartridgeType::ROM_RAM_BATTERY),
            0x0B => Some(CartridgeType::MMM01),
            0x0C => Some(CartridgeType::MMM01_RAM),
            0x0D => Some(CartridgeType::MMM01_RAM_BATTERY),
            0x0F => Some(CartridgeType::MBC3_TIMER_BATTERY),
            0x10 => Some(CartridgeType::MBC3_TIMER_RAM_BATTERY),
            0x11 => Some(CartridgeType::MBC3),
            0x12 => Some(CartridgeType::MBC3_RAM),
            0x13 => Some(CartridgeType::MBC3_RAM_BATTERY),
            0x19 => Some(CartridgeType::MBC5),
            0x1A => Some(CartridgeType::MBC5_RAM),
            0x1B => Some(CartridgeType::MBC5_RAM_BATTERY),
            0x1C => Some(CartridgeType::MBC5_RUMBLE),
            0x1D => Some(CartridgeType::MBC5_RUMBLE_RAM),
            0x1E => Some(CartridgeType::MBC5_RUMBLE_RAM_BATTERY),
            0x20 => Some(CartridgeType::MBC6),
            0x22 => Some(CartridgeType::MBC7_SENSOR_RUMBLE_RAM_BATTERY),
            0xFC => Some(CartridgeType::POCKET_CAMERA),
            0xFD => Some(CartridgeType::BANDAI_TAMA5),
            0xFE => Some(CartridgeType::HuC3),
            0xFF => Some(CartridgeType::HuC1_RAM_BATTERY),
            _ => None,
        }
    }
}
