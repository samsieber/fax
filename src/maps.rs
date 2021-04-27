use fax_derive::bitmaps;
use crate::{BitReader, Bits};

enum Entry<T: Copy + 'static> {
    Empty,
    Leaf(u8, &'static [Option<(T, u8)>]),
    Value(T, u8),
    Prefix(u8, &'static [Entry<T>])
}
impl<T: Copy> Entry<T> {
    fn find(&self, reader: &mut impl BitReader) -> Option<T> {
        match *self {
            Entry::Empty => None,
            Entry::Value(val, len) => {
                reader.consume(len);
                Some(val)
            }
            Entry::Leaf(width, lut) => {
                let index = reader.peek(width)?;
                let (val, len) = lut[index as usize]?;
                reader.consume(len);
                Some(val)
            }
            Entry::Prefix(width, lut) => {
                let index = reader.peek(width)?;
                let entry = &lut[index as usize];
                match *entry {
                    Entry::Empty => None,
                    Entry::Value(val, len) => {
                        reader.consume(len);
                        Some(val)
                    }
                    _ => {
                        reader.consume(width);
                        entry.find(reader)
                    }
                }
            }
        }
    }
}

pub const EDFB_HALF: Bits = Bits { data: 1, len: 12 };
pub const EOL: Bits = Bits { data: 1, len: 12 };

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Pass,
    Horizontal,
    Vertical(i8),
    Extension,
}

bitmaps! {
    mode <Mode> {
        0001 => Mode::Pass,
        001 => Mode::Horizontal,
        1 => Mode::Vertical(0),
        011 => Mode::Vertical(1),
        000011 => Mode::Vertical(2),
        0000011 => Mode::Vertical(3),
        010 => Mode::Vertical(-1),
        000010 => Mode::Vertical(-2),
        0000010 => Mode::Vertical(-3),
        0000001 => Mode::Extension,
    },
    black {
        0000110111 => 0,
        010 => 1,
        11 => 2,
        10 => 3,
        011 => 4,
        0011 => 5,
        0010 => 6,
        00011 => 7,
        000101 => 8,
        000100 => 9,
        0000100 => 10,
        0000101 => 11,
        0000111 => 12,
        00000100 => 13,
        00000111 => 14,
        000011000 => 15,
        0000010111 => 16,
        0000011000 => 17,
        0000001000 => 18,
        00001100111 => 19,
        00001101000 => 20,
        00001101100 => 21,
        00000110111 => 22,
        00000101000 => 23,
        00000010111 => 24,
        00000011000 => 25,
        000011001010 => 26,
        000011001011 => 27,
        000011001100 => 28,
        000011001101 => 29,
        000001101000 => 30,
        000001101001 => 31,
        000001101010 => 32,
        000001101011 => 33,
        000011010010 => 34,
        000011010011 => 35,
        000011010100 => 36,
        000011010101 => 37,
        000011010110 => 38,
        000011010111 => 39,
        000001101100 => 40,
        000001101101 => 41,
        000011011010 => 42,
        000011011011 => 43,
        000001010100 => 44,
        000001010101 => 45,
        000001010110 => 46,
        000001010111 => 47,
        000001100100 => 48,
        000001100101 => 49,
        000001010010 => 50,
        000001010011 => 51,
        000000100100 => 52,
        000000110111 => 53,
        000000111000 => 54,
        000000100111 => 55,
        000000101000 => 56,
        000001011000 => 57,
        000001011001 => 58,
        000000101011 => 59,
        000000101100 => 60,
        000001011010 => 61,
        000001100110 => 62,
        000001100111 => 63,
        0000001111 => 64,
        000011001000 => 128,
        000011001001 => 192,
        000001011011 => 256,
        000000110011 => 320,
        000000110100 => 384,
        000000110101 => 448,
        0000001101100 => 512,
        0000001101101 => 576,
        0000001001010 => 640,
        0000001001011 => 704,
        0000001001100 => 768,
        0000001001101 => 832,
        0000001110010 => 896,
        0000001110011 => 960,
        0000001110100 => 1024,
        0000001110101 => 1088,
        0000001110110 => 1152,
        0000001110111 => 1216,
        0000001010010 => 1280,
        0000001010011 => 1344,
        0000001010100 => 1408,
        0000001010101 => 1472,
        0000001011010 => 1536,
        0000001011011 => 1600,
        0000001100100 => 1664,
        0000001100101 => 1728,
        00000001000 => 1792,
        00000001100 => 1856,
        00000001101 => 1920,
        000000010010 => 1984,
        000000010011 => 2048,
        000000010100 => 2112,
        000000010101 => 2176,
        000000010110 => 2240,
        000000010111 => 2304,
        000000011100 => 2368,
        000000011101 => 2432,
        000000011110 => 2496,
        000000011111 => 2560,
    },
    white {
        00110101 => 0,
        000111 => 1,
        0111 => 2,
        1000 => 3,
        1011 => 4,
        1100 => 5,
        1110 => 6,
        1111 => 7,
        10011 => 8,
        10100 => 9,
        00111 => 10,
        01000 => 11,
        001000 => 12,
        000011 => 13,
        110100 => 14,
        110101 => 15,
        101010 => 16,
        101011 => 17,
        0100111 => 18,
        0001100 => 19,
        0001000 => 20,
        0010111 => 21,
        0000011 => 22,
        0000100 => 23,
        0101000 => 24,
        0101011 => 25,
        0010011 => 26,
        0100100 => 27,
        0011000 => 28,
        00000010 => 29,
        00000011 => 30,
        00011010 => 31,
        00011011 => 32,
        00010010 => 33,
        00010011 => 34,
        00010100 => 35,
        00010101 => 36,
        00010110 => 37,
        00010111 => 38,
        00101000 => 39,
        00101001 => 40,
        00101010 => 41,
        00101011 => 42,
        00101100 => 43,
        00101101 => 44,
        00000100 => 45,
        00000101 => 46,
        00001010 => 47,
        00001011 => 48,
        01010010 => 49,
        01010011 => 50,
        01010100 => 51,
        01010101 => 52,
        00100100 => 53,
        00100101 => 54,
        01011000 => 55,
        01011001 => 56,
        01011010 => 57,
        01011011 => 58,
        01001010 => 59,
        01001011 => 60,
        00110010 => 61,
        00110011 => 62,
        00110100 => 63,
        11011 => 64,
        10010 => 128,
        010111 => 192,
        0110111 => 256,
        00110110 => 320,
        00110111 => 384,
        01100100 => 448,
        01100101 => 512,
        01101000 => 576,
        01100111 => 640,
        011001100 => 704,
        011001101 => 768,
        011010010 => 832,
        011010011 => 896,
        011010100 => 960,
        011010101 => 1024,
        011010110 => 1088,
        011010111 => 1152,
        011011000 => 1216,
        011011001 => 1280,
        011011010 => 1344,
        011011011 => 1408,
        010011000 => 1472,
        010011001 => 1536,
        010011010 => 1600,
        011000 => 1664,
        010011011 => 1728,
        00000001000 => 1792,
        00000001100 => 1856,
        00000001101 => 1920,
        000000010010 => 1984,
        000000010011 => 2048,
        000000010100 => 2112,
        000000010101 => 2176,
        000000010110 => 2240,
        000000010111 => 2304,
        000000011100 => 2368,
        000000011101 => 2432,
        000000011110 => 2496,
        000000011111 => 2560,
    },
}

