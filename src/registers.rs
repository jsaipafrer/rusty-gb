const ZERO_FLAG: u8 = 1 << 7;
const SUBTRACT_FLAG: u8 = 1 << 6;
const HALF_CARRY_FLAG: u8 = 1 << 5;
const CARRY_FLAG: u8 = 1 << 4;

/// CPU's registers
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

/// CPU allows fetching data from pairs of registers as a single 16b register
/// Functions below implement this behaviour
impl Registers {
    // AF
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(&self.f) as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagsRegister::from(value as u8)
    }

    // BC
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    // DE
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    // HL
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

/// Struct representation for special flags register
/// Format "zshc 0000" where z = zero, s = subtraction, h = half carry, c = carry
/// Lower nibble is always 0, so we just represent the upper one
struct FlagsRegister {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

// make possible to convert FlagsRegister <-> u8
impl From<FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> Self {
        (if flag.zero           { ZERO_FLAG } else { 0 }) |
        (if flag.subtraction    { SUBTRACT_FLAG } else { 0 }) |
        (if flag.half_carry     { HALF_CARRY_FLAG } else { 0 })  |
        (if flag.carry          { CARRY_FLAG } else { 0 })
    }
}

impl From<u8> for FlagsRegister {
    fn from(register: &u8) -> Self {
        FlagsRegister{
            zero: register & ZERO_FLAG == 1,
            subtraction: register & SUBTRACT_FLAG == 1,
            half_carry: register & HALF_CARRY_FLAG == 1,
            carry: register & CARRY_FLAG == 1,
        }
    }
}