use crate::{ Alphabate, Encode, Decode };
use std::iter::FromIterator;

// Alphabate of Base32 and Base32Hex
pub struct Base32;
const BASE32_ENCODE_ALPHABATE: [char; 32] = [
     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
     'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
     'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
     'Y', 'Z', '2', '3', '4', '5', '6', '7',
];
const BASE32_DECODE_ALPHABATE: [u8; 256] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
//                                                                    '+' ,       '-' ,       '/' ,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
//  '0' , '1' , '2' , '3',  '4' , '5' , '6' , '7' , '8' , '9' ,
    0xff, 0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0xff, 0xff,
//       'A' , 'B' , 'C' , 'D',  'E' , 'F' , 'G' , 'H' , 'I' , 'J' , 'K' , 'L' , 'M' , 'N' ,  'O' ,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
//  'P' , 'Q' , 'R' , 'S' , 'T' , 'U',  'V' , 'W' , 'X' , 'Y' , 'Z' ,                         '_' ,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

pub struct Base32Hex;
const BASE32HEX_ENCODE_ALPHABATE: [char; 32] = [
     '0', '1', '2', '3', '4', '5', '6', '7',
     '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
     'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
     'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
];
const BASE32HEX_DECODE_ALPHABATE: [u8; 256] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
//                                                                    '+' ,       '-' ,       '/' ,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
//  '0' , '1' , '2' , '3',  '4' , '5' , '6' , '7' , '8' , '9' ,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xff, 0xff, 0xff, 0x00, 0xff, 0xff,
//       'A' , 'B' , 'C' , 'D',  'E' , 'F' , 'G' , 'H' , 'I' , 'J' , 'K' , 'L' , 'M' , 'N' ,  'O' ,
    0xff, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
//  'P' , 'Q' , 'R' , 'S' , 'T' , 'U',  'V' , 'W' , 'X' , 'Y' , 'Z' ,                         '_' ,
    0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

impl Alphabate for Base32 {
    fn get_character_from_index(index: u8) -> Option<char> {
        Some(BASE32_ENCODE_ALPHABATE[index as usize])
    }
    fn get_index_from_character(character: char) -> Option<u8> {
        Some(BASE32_DECODE_ALPHABATE[character as usize])
    }
}

impl Alphabate for Base32Hex {
    fn get_character_from_index(index: u8) -> Option<char> {
        Some(BASE32HEX_ENCODE_ALPHABATE[index as usize])
    }
    fn get_index_from_character(character: char) -> Option<u8> {
        Some(BASE32HEX_DECODE_ALPHABATE[character as usize])
    }
}

// Encoder and Decoder for Base32 or Base32Hex
struct Encoder<T: Alphabate>(T);
impl<T: Alphabate> Encoder<T> {
    fn split(chunk: &[u8]) -> Vec<u8> {
        match chunk.len() {
            1 => vec![
                chunk[0] >> 3,
                (chunk[0] & 0b0000_0111) << 2,
            ],
            2 => vec![
                chunk[0] >> 3,
                (chunk[0] & 0b0000_0111) << 2 | chunk[1] >> 6,
                (chunk[1] & 0b0011_1111) >> 1,
                (chunk[1] & 0b0000_0001) << 4,
            ],
            3 => vec![
                chunk[0] >> 3,
                (chunk[0] & 0b0000_0111) << 2 | chunk[1] >> 6,
                (chunk[1] & 0b0011_1111) >> 1,
                (chunk[1] & 0b0000_0001) << 4 | (chunk[2] & 0b1111_0000) >> 4,
                (chunk[2] & 0b0000_1111) << 1,
            ],
            4 => vec![
                chunk[0] >> 3,
                (chunk[0] & 0b0000_0111) << 2 | chunk[1] >> 6,
                (chunk[1] & 0b0011_1111) >> 1,
                (chunk[1] & 0b0000_0001) << 4 | (chunk[2] & 0b1111_0000) >> 4,
                (chunk[2] & 0b0000_1111) << 1 | (chunk[3] & 0b1000_0000) >> 7,
                (chunk[3] & 0b0111_1100) >> 2,
                (chunk[3] & 0b0000_0011) << 3,
            ],
            5 => vec![
                chunk[0] >> 3,
                (chunk[0] & 0b0000_0111) << 2 | chunk[1] >> 6,
                (chunk[1] & 0b0011_1111) >> 1,
                (chunk[1] & 0b0000_0001) << 4 | (chunk[2] & 0b1111_0000) >> 4,
                (chunk[2] & 0b0000_1111) << 1 | (chunk[3] & 0b1000_0000) >> 7,
                (chunk[3] & 0b0111_1100) >> 2,
                (chunk[3] & 0b0000_0011) << 3 | (chunk[4] & 0b1110_0000) >> 5,
                (chunk[4] & 0b0001_1111),
            ],
            _ => unreachable!(),
        }
    }
    fn encode_chunk(chunk: &[u8]) -> Vec<char> {
        let mut out = [T::pad(); 8];
        for i in 0..chunk.len() {
            if let Some(character) = 
                T::get_character_from_index(chunk[i]) {
                    out[i] = character;
                }
        }
        return out.to_vec();
    }
}

struct Decoder<T: Alphabate>(T);
impl<T: Alphabate> Decoder<T> {
    fn stitch(chunk: &[u8]) -> Vec<u8> {
        let out = match chunk.len() {
            2 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6,
            ],
            3 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1,
            ],
            4 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1 | (chunk[3] & 0b0001_0000) >> 4,
                (chunk[3] & 0b0000_1111) << 4,
            ],
            5 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1 | (chunk[3] & 0b0001_0000) >> 4,
                (chunk[3] & 0b0000_1111) << 4 | (chunk[4] & 0b0001_1110) >> 1,
                (chunk[4] & 0b0000_0001) << 7,
            ],
            6 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1 | (chunk[3] & 0b0001_0000) >> 4,
                (chunk[3] & 0b0000_1111) << 4 | (chunk[4] & 0b0001_1110) >> 1,
                (chunk[4] & 0b0000_0001) << 7 | chunk[5] << 2,
            ],
            7 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1 | (chunk[3] & 0b0001_0000) >> 4,
                (chunk[3] & 0b0000_1111) << 4 | (chunk[4] & 0b0001_1110) >> 1,
                (chunk[4] & 0b0000_0001) << 7 | chunk[5] << 2 | (chunk[6] & 0b0001_1000) >> 3,
                (chunk[6] & 0b0000_0111) << 5,
            ],
            8 => vec![
                chunk[0] << 3 | (chunk[1] & 0b0001_1100) >> 2,
                (chunk[1] & 0b0000_0011) << 6 | chunk[2] << 1 | (chunk[3] & 0b0001_0000) >> 4,
                (chunk[3] & 0b0000_1111) << 4 | (chunk[4] & 0b0001_1110) >> 1,
                (chunk[4] & 0b0000_0001) << 7 | chunk[5] << 2 | (chunk[6] & 0b0001_1000) >> 3,
                (chunk[6] & 0b0000_0111) << 5 | chunk[7],
            ],
            _ => unreachable!(),
        };
        return out.into_iter().filter(|&x| x > 0).collect();
    }
    fn decode_chunk(chunk: &[char]) -> Vec<u8> {
        chunk
            .iter()
            .filter(|character| **character != T::pad())
            .map(|character| {
                T::get_index_from_character(*character).unwrap()
            })
            .collect()
    }
}

// impl trait for Base32 or Base32Hex
impl Encode for Base32 {
    fn encode(data: &[u8]) -> String {
        let encoded = data
            .chunks(5)
            .map(|chunk| Encoder::<Self>::split(&chunk))
            .flat_map(|chunk| Encoder::<Self>::encode_chunk(&chunk));
        return String::from_iter(encoded);
    }
}
impl Encode for Base32Hex {
    fn encode(data: &[u8]) -> String {
        let encoded = data
            .chunks(5)
            .map(|chunk| Encoder::<Self>::split(&chunk))
            .flat_map(|chunk| Encoder::<Self>::encode_chunk(&chunk));
        return String::from_iter(encoded);
    }
}

impl Decode for Base32 {
    fn decode(code: &str) -> Vec<u8> {
        if code.chars().count() % 8 != 0 {
            println!("Error: Need padding")
        }
        code
            .chars()
            .collect::<Vec<char>>()
            .chunks(8)
            .map(|chunk| Decoder::<Self>::decode_chunk(&chunk))
            .flat_map(|chunk| Decoder::<Self>::stitch(&chunk))
            .collect()
    }
}
impl Decode for Base32Hex {
    fn decode(code: &str) -> Vec<u8> {
        if code.chars().count() % 8 != 0 {
            println!("Error: Need padding")
        }
        code
            .chars()
            .collect::<Vec<char>>()
            .chunks(8)
            .map(|chunk| Decoder::<Self>::decode_chunk(&chunk))
            .flat_map(|chunk| Decoder::<Self>::stitch(&chunk))
            .collect()
    }
}
