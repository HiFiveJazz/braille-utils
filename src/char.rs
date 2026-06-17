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

    pub const fn english_to_bits(c: char) -> Option<u8> {
        match c {
            'a' | 'A' => Some(0b000001), // ⠁
            'b' | 'B' => Some(0b000011), // ⠃
            'c' | 'C' => Some(0b001001), // ⠉
            'd' | 'D' => Some(0b011001), // ⠙
            'e' | 'E' => Some(0b010001), // ⠑
            'f' | 'F' => Some(0b001011), // ⠋
            'g' | 'G' => Some(0b011011), // ⠛
            'h' | 'H' => Some(0b010011), // ⠓
            'i' | 'I' => Some(0b001010), // ⠊
            'j' | 'J' => Some(0b011010), // ⠚

            'k' | 'K' => Some(0b000101), // ⠅
            'l' | 'L' => Some(0b000111), // ⠇
            'm' | 'M' => Some(0b001101), // ⠍
            'n' | 'N' => Some(0b011101), // ⠝
            'o' | 'O' => Some(0b010101), // ⠕
            'p' | 'P' => Some(0b001111), // ⠏
            'q' | 'Q' => Some(0b011111), // ⠟
            'r' | 'R' => Some(0b010111), // ⠗
            's' | 'S' => Some(0b001110), // ⠎
            't' | 'T' => Some(0b011110), // ⠞

            'u' | 'U' => Some(0b100101), // ⠥
            'v' | 'V' => Some(0b100111), // ⠧
            'w' | 'W' => Some(0b111010), // ⠺
            'x' | 'X' => Some(0b101101), // ⠭
            'y' | 'Y' => Some(0b111101), // ⠽
            'z' | 'Z' => Some(0b110101), // ⠵
            _ => None,
        }
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
