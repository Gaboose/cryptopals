use std::env;

mod conversions;
use conversions::{FromHex, ToHex, BytesToString};

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
    println!("text: {}", bytes1.bytes_to_string());

    println!("\n2nd string");
    println!("hex: {}", hex2);
    println!("text: {}", bytes2.bytes_to_string());

    let xored_bytes: Vec<_> = bytes1.iter().zip(bytes2).map(|(b1, b2)| b1 ^ b2).collect();

    println!("\nxored strings");
    println!("hex: {}", xored_bytes.to_hex());
    println!("text: {}", xored_bytes.bytes_to_string());
}
