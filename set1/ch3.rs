use std::env;

mod conversions;
use conversions::{FromHex, ToHex, BytesToString};

fn count_chars<I>(bytes: I) -> u32 where I: Iterator<Item=u8> {
    let mut count = 0;
    for byte in bytes {
        match byte {
            b'a'...b'z' | b'A'...b'Z' | b' ' => count += 1,
            _ => {}
        }
    }
    count
}

fn main() {
    let hex = match env::args().nth(1) {
        Some(arg) => arg,
        None => "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()
    };
    let bytes = hex.from_hex();

    println!("cipher");
    println!("hex: {}", hex);
    println!("text: {}", bytes.bytes_to_string());

    let scores: Vec<_> = (0..0xFF).map(|key| {
        let candidate = bytes.iter().map(|byte| byte ^ key);
        count_chars(candidate)
    }).collect();

    let mut sorted_scores: Vec<_> = (0..0xFFu8).zip(scores).collect();
    sorted_scores.sort_by(|&(_, el1), &(_, el2)| el2.cmp(&el1));

    let top_scores = sorted_scores.iter().take(10);

    println!("\ntop {} keys", top_scores.len());
    print!("score:");
    for &(_key, score) in top_scores.clone() {
        print!(" {}", score);
    }
    print!("\nhex:");
    for &(key, _score) in top_scores.clone() {
        print!(" {}", vec![key].to_hex());
    }
    print!("\ntext:");
    for &(key, _score) in top_scores {
        print!(" {}", key as char);
    }

    let (key, _score) = sorted_scores[0];
    let message: Vec<u8> = bytes.iter().map(|&byte| byte ^ key).collect();

    println!("\n\nmessage");
    println!("hex: {}", message.to_hex());
    println!("text: {}", message.bytes_to_string());
}
