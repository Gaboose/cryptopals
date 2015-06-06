use std::{env, str};

mod bases;
use bases::{FromHex, ToHex};

fn main() {
    let hex1 = match env::args().nth(1) {
        Some(arg) => arg,
        None => "1c0111001f010100061a024b53535009181c".to_string()
    };
    let hex2 = match env::args().nth(2) {
        Some(arg) => arg,
        None => "686974207468652062756c6c277320657965".to_string()
    };
    let (bytes1, bytes2) = (hex1.from_hex(), hex2.from_hex());

    println!("1st string");
    println!("hex: {}", hex1);
    println!("text: {}", str::from_utf8(&bytes1).unwrap());

    println!("\n2nd string");
    println!("hex: {}", hex2);
    println!("text: {}", str::from_utf8(&bytes2).unwrap());

    let xor_bytes: Vec<_> = bytes1.iter().zip(bytes2).map(|(b1, b2)| b1 ^ b2).collect();

    println!("\nxored strings");
    println!("hex: {}", xor_bytes.to_hex());
    println!("text: {}", str::from_utf8(&xor_bytes).unwrap());
}
