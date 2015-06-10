use std::io::prelude::*;
use std::fs::File;

mod conversions;
use conversions::{FromHex, BytesToString};

fn count_chars<'a, I>(bytes: I) -> u32 where I: Iterator<Item=&'a u8> {
    bytes.fold(0, |count, byte| {
        match *byte {
            b'a'...b'z' | b'A'...b'Z' | b' ' => count + 1,
            _ => count
        }
    })
}

struct LineResult {
    hex: String,
    text: String,
    key: u8,
    score: u32
}

fn main() {
    let mut file = File::open("ch4.txt").unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    let lines = data.split('\n').collect::<Vec<_>>();

    let mut results = lines.iter().map(|string| {
        let bytes = string.from_hex();

        let mut best_result = LineResult{
            hex: string.to_string(),
            text: String::new(),
            key: 0,
            score: 0
        };

        for key in 0..0xFF {
            let candidate = bytes.iter().map(|byte| byte ^ key).collect::<Vec<u8>>();
            let score = count_chars(candidate.iter());
            if score > best_result.score {
                best_result.score = score;
                best_result.key = key;
                best_result.text = candidate.bytes_to_string();
            }
        }
        return best_result;
    }).collect::<Vec<_>>();

    results.sort_by(|el1, el2| el2.score.cmp(&el1.score));

    let top = results.iter().take(10);
    print!("top {} scores:", top.len());
    for result in top {
        print!(" {}", result.score);
    }
    println!("");

    for i in 0..3 {
        println!("");
        match i {
            0 => println!("the winner"),
            1 => println!("the 1st runner-up"),
            2 => println!("the 2nd runner-up"),
            _ => unreachable!()
        }
        println!("cipher: {}", results[i].hex);
        println!("message: {}", results[i].text);
    }
}
