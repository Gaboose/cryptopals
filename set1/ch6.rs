use std::io::prelude::*;
use std::fs::File;

mod conversions;
use conversions::{FromBase64, BytesToString};

fn distance(x: u8, y: u8) -> u8 {
    let mut count = 0;
    let mut xor = x ^ y;
    for _ in 0..8 {
        count += xor % 2;
        xor >>= 1;
    }
    count
}

fn distance_array(x: &[u8], y: &[u8]) -> u8 {
    x.iter().zip(y).fold(0, |acc, (&x, &y)| acc + distance(x, y))
}

fn transpose<T: Copy>(matrix: &[&[T]]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..matrix[0].len() {
        let mut column = Vec::new();
        for row in matrix {
            if i < row.len() {
                column.push(row[i]);
            }
        }
        result.push(column);
    }
    return result;
}

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

fn get_keybyte(cipher: &[u8]) -> u8 {
    let scores: Vec<_> = (0..0xFF).map(|key| {
        let candidate = cipher.iter().map(|byte| byte ^ key);
        count_chars(candidate)
    }).collect();

    let mut sorted_scores: Vec<_> = (0..0xFF).zip(scores).collect();
    sorted_scores.sort_by(|&(_, el1), &(_, el2)| el2.cmp(&el1));

    return sorted_scores[0].0;
}

fn main() {
    let mut file = File::open("ch6.txt").unwrap();
    let mut cipher = String::new();
    let _ = file.read_to_string(&mut cipher);
    let cipher = cipher.replace("\n", "").from_base64();

    // Find the hamming distances for all keysizes
    let mut keysize_distances = (2..40).map(|keysize| {
        let dists: Vec<u8> = cipher.chunks(keysize).collect::<Vec<&[u8]>>()
                                   .chunks(2).flat_map(|two| {
            match two.len() {
                2 => vec![distance_array(&two[0], &two[1])].into_iter(),
                _ => vec![].into_iter()
            }
        }).collect();

        let sum = dists.iter().fold(0u32, |acc, &x| acc + x as u32);
        let normalized = sum as f32 / dists.len() as f32 / keysize as f32;
        (keysize, normalized)
    }).collect::<Vec<_>>();
    keysize_distances.sort_by(|el1, el2| el1.1.partial_cmp(&el2.1).unwrap());

    // Print the top 5 keysizes
    let top = keysize_distances.iter().take(5);
    println!("top {}", top.len());
    println!("keysize\tnormalized hamming distance");
    for &(keysize, dist) in top {
        println!("{}\t{:0.2}", keysize, dist)
    }

    // Find the key
    let (keysize, _) = keysize_distances[0];
    let mut key: Vec<u8> = Vec::new();
    print!("\nkey: ");
    for column in transpose(&cipher.chunks(keysize).collect::<Vec<&[u8]>>()) {
        let keybyte = get_keybyte(&column);
        key.push(keybyte);
        print!("{}", keybyte as char);
    }
    println!("");

    // Decrypt the message
    let message: Vec<u8> = cipher.iter().zip(key.iter().cycle()).map(|(x, y)| x ^ y).collect();

    println!("\nmessage\n{}", message.bytes_to_string());
}
