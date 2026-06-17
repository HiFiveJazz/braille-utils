#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrailleChar {
    bits: u8,
}

// A 6-dot braille cell stored in the lower 6 bits of a u8.
//
// Braille dots:
//
//     dot 1   dot 4
//     dot 2   dot 5
//     dot 3   dot 6
//
// Bit layout:
//
//     bit 0   bit 3
//     bit 1   bit 4
//     bit 2   bit 5
//
// So the byte is:
//
//     0b00_543210
//
// Bits 6 and 7 are unused. We use u8 because Rust does not have a u6 type.
impl BrailleChar {
    pub const UNICODE_BASE: u32 = 0x2800;

    pub const fn from_bits(bits: u8) -> Option<Self> {
        if bits <= 0b0011_1111 {
            Some(Self { bits })
        } else {
            None
        }
    }

    pub const fn bits(self) -> u8 {
        self.bits
    }

    pub fn to_unicode(self) -> char {
        char::from_u32(Self::UNICODE_BASE + self.bits as u32).unwrap()
    }

    pub fn from_unicode(c: char) -> Option<Self> {
        let code = c as u32;

        if code < Self::UNICODE_BASE {
            return None;
        }

        let bits = code - Self::UNICODE_BASE;

        if bits <= 0b0011_1111 {
            Some(Self { bits: bits as u8 })
        } else {
            None
        }
    }
}
