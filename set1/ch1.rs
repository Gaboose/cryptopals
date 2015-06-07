use std::env;

mod conversions;
use conversions::{FromHex, ToBase64, BytesToString};

fn main() {
    let hex = match env::args().nth(1) {
        Some(arg1) => arg1,
        None => "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()
    };
    println!("hex: {}", hex);
    let bytes = hex.from_hex();
    println!("text: {}", bytes.bytes_to_string());
    let base64 = bytes.to_base64();
    println!("base64: {}", base64);
}
