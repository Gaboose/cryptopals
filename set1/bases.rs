use std::str;

pub trait FromHex {
    fn from_hex(&self) -> Vec<u8>;
}

impl FromHex for str {
    fn from_hex(&self) -> Vec<u8> {
        self.as_bytes().iter().map(|&byte| {
            match byte {
                b'0'...b'9' => byte - b'0',
                b'a'...b'f' => byte - b'a' + 10,
                _ => panic!("That's invalid hex you're feeding me!")
            }
        }).collect::<Vec<_>>().chunks(2).map(|chunk| {
            chunk[0] << 4 | chunk[1]
        }).collect()
    }
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for Vec<u8> {
    fn to_hex(&self) -> String {
        let symbols: Vec<u8> = self.iter()
            .flat_map(|&byte| vec![byte >> 4, byte & 0xF].into_iter())
            .map(|symbol| {
                match symbol {
                    0...9 => b'0' + symbol,
                    10...15 => b'a' + symbol - 10,
                    _ => b'?'
                }
            }).collect();
        str::from_utf8(&symbols).unwrap().to_string()
    }
}

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Vec<u8> {
    fn to_base64(&self) -> String {

        fn symbols_to_string(vector: Vec<u8>) -> String {
            let mut result = String::with_capacity(vector.len());
            for sixtet in vector {
                let c = match sixtet {
                    0...25 => b'A' + sixtet,
                    26...51 => b'a' + sixtet - 26,
                    52...61 => b'0' + sixtet - 52,
                    62 => b'+',
                    63 => b'/',
                    _ => b'?'
                };
                result.push(c as char);
            }
            return result;
        }

        let mut result: String = String::with_capacity(self.len() / 2);
        let mut modulus = 0;
        let mut buf: u32 = 0;

        for byte in self {
            buf = buf << 8 | *byte as u32;
            modulus += 1;
            if modulus == 3 {
                result.push_str(&symbols_to_string(vec![
                    (buf >> 18 & 0b0011_1111) as u8,
                    (buf >> 12 & 0b0011_1111) as u8,
                    (buf >> 6 & 0b0011_1111) as u8,
                    (buf & 0b0011_1111) as u8
                ]));
                modulus = 0;
            }
        }

        let tail = match modulus {
            1 => symbols_to_string(vec![
                (buf >> 2 & 0b0011_1111) as u8,
                (buf << 4 & 0b0011_0000) as u8
            ]) + "==",
            2 => symbols_to_string(vec![
                (buf >> 10 & 0b0011_1111) as u8,
                (buf >> 4 & 0b0011_1111) as u8,
                (buf << 2 & 0b0011_1100) as u8
            ]) + "=",
            _ => String::new()
        };

        result.push_str(&tail);
        return result;
    }
}
