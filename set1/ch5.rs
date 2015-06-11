use std::env;

mod conversions;
use conversions::{ToHex};

fn main() {
    let text = match env::args().nth(1) {
        Some(arg) => arg,
        None => "Burning 'em, if you ain't quick and nimble\n\
                 I go crazy when I hear a cymbal".to_string()
    };
    let key = match env::args().nth(2) {
        Some(arg) => arg,
        None => "ICE".to_string()
    };

    let cipher:Vec<u8> = text.bytes().zip(key.bytes().cycle()).map(|(x, y)| x ^ y).collect();

    println!("{}", cipher.to_hex())
}
