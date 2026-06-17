#![no_std]

#[cfg(test)]
extern crate std;

mod char;
mod conversions;

pub use char::BrailleChar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_braille_char() {
        let b = BrailleChar::from_bits(0b000001).unwrap();

        std::dbg!(b);

        assert_eq!(b.bits(), 0b000001);
    }
}
