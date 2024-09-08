#[derive(Debug)]
pub struct U6 {
    value: u8,
}

impl U6 {
    pub fn from(val: u8) -> Self {
        U6 { value: val & 0x3f }
    }

    pub fn to_u8(&self) -> u8 {
        self.value
    }
}

#[derive(Debug)]
pub struct U4 {
    value: u8,
}

impl U4 {
    pub fn from(val: u8) -> Self {
        U4 { value: val & 0xf }
    }

    pub fn to_u8(&self) -> u8 {
        self.value
    }
}
