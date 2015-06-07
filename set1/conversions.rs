use std::{char, u8};

pub trait BytesToString {
    fn bytes_to_string(&self) -> String;
}

impl BytesToString for [u8] {
    fn bytes_to_string(&self) -> String {
        // Considered using str::from_utf8(&self).unwrap().to_string() instead,
        // but we need something that works on non-utf8 slices.
        self.iter().fold(String::new(), |mut acc, &byte| {acc.push(byte as char); acc})
    }
}

pub trait FromHex {
    fn from_hex(&self) -> Vec<u8>;
}

impl FromHex for str {
    fn from_hex(&self) -> Vec<u8> {
        self.as_bytes().chunks(2)
            .map(|chunk| u8::from_str_radix(&chunk.bytes_to_string(), 16).unwrap())
            .collect()
    }
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        self.iter()
            .flat_map(|&byte| vec![byte >> 4, byte & 0xF].into_iter())
            .map(|symbol| char::from_digit(symbol as u32, 16).unwrap())
            .collect()
    }
}

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for [u8] {
    fn to_base64(&self) -> String {
        fn symbols_to_string(vector: Vec<u8>) -> String {
            vector.iter().map(|sixtet| {
                (match sixtet & 0b0011_1111 {
                    sixtet @ 0...25 => b'A' + sixtet,
                    sixtet @ 26...51 => b'a' + sixtet - 26,
                    sixtet @ 52...61 => b'0' + sixtet - 52,
                    62 => b'+',
                    63 => b'/',
                    _ => b'?'
                }) as char
            }).collect()
        }

        self.chunks(3).map(|chunk| {
            match chunk.len() {
                3 => symbols_to_string(vec![
                    chunk[0] >> 2,
                    chunk[0] << 4 | chunk[1] >> 4,
                    chunk[1] << 2 | chunk[2] >> 6,
                    chunk[2]
                ]),
                2 => symbols_to_string(vec![
                    chunk[0] >> 2,
                    chunk[0] << 4 | chunk[1] >> 4,
                    chunk[1] << 2
                ]) + "=",
                1 => symbols_to_string(vec![
                    chunk[0] >> 2,
                    chunk[0] << 4
                ]) + "==",
                _ => panic!("chunks(3) is broken")
            }
        }).fold(String::new(), |acc, x| acc + &x)
    }
}
