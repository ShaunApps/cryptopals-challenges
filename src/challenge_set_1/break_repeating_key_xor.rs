use base64;
use challenge_set_1::bit_vec::BitVec;
use challenge_set_1::single_byte_xor_cipher::rank_char_frequency;
use hex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str;

fn solve_blocks(blocks: Vec<Vec<u8>>) -> Vec<u8> {
    let mut key_vec = Vec::new();
    for i in blocks.iter() {
        key_vec.push(single_byte_xor_cipher(i))
    }
    key_vec
}

pub fn single_byte_xor_cipher(data: &Vec<u8>) -> u8 {
    let mut top_score: i32 = 0;
    let mut phrase: String = "".to_string();
    let mut top_byte = 0;
    for byte in 0..=255 {
        let xored: Vec<_> = data.iter().map(|&x| x ^ byte).collect();
        let char_score = rank_char_frequency(&xored);
        if char_score > top_score {
            top_score = char_score;
            top_byte = byte;
            let thing = String::from_utf8(xored);
            phrase = match thing {
                Ok(x) => x,
                Err(error) => String::from(""),
            };
        }
    }
    top_byte
}

fn break_ciphertext_into_blocks(keysize: u8) -> Vec<Vec<u8>> {
    let mut f = File::open("1_challenge_6.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let decoded_f = base64::decode(&contents).unwrap();

    // The below commented code was my first attempt at getting the transposed blocks
    // trans_vec was returning an empty Vec<Vec<u8>>, might be something with map

    // let mut iter = decoded_f.chunks(keysize as usize);
    // let mut block_vec = Vec::new();
    // for i in iter {
    //     block_vec.push(i);
    // }

    // let mut trans_vec = Vec::new();
    // for i in 0..keysize {
    //     let mut level_vec = Vec::new();
    //     block_vec.iter().map(|&x| {
    //         level_vec.push(x[i as usize]);
    //     });
    //     trans_vec.push(level_vec)
    // }
    // trans_vec

    let mut trans_vec: Vec<Vec<u8>> = (0..keysize).map(|_| Vec::new()).collect();
    for block in decoded_f.chunks(keysize as usize) {
        for (&u, bt) in block.iter().zip(trans_vec.iter_mut()) {
            bt.push(u);
        }
    }
    trans_vec
}

fn find_smallest_KEYSIZE() -> Vec<u8> {
    let mut key_map: HashMap<usize, u8> = HashMap::new();

    for i in 1..=40 {
        let key_size = (100f64 * normalized_KEYSIZE(i)) as usize;
        key_map.insert(key_size, i);
    }
    let mut keys = Vec::new();
    for (key, value) in key_map.iter() {
        keys.push(key);
    }
    keys.sort();
    // we want to try 3 keys
    let value1 = key_map.get(keys[0]).unwrap().to_owned();
    let value2 = key_map.get(keys[1]).unwrap().to_owned();
    let value3 = key_map.get(keys[2]).unwrap().to_owned();
    vec![value1, value2, value3]
}

// work on fixing this
fn normalized_KEYSIZE(key_guess: u8) -> f64 {
    let mut f = File::open("1_challenge_6.txt").unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let decoded_f = base64::decode(&contents).unwrap();
    let mut iter = decoded_f.chunks(key_guess as usize);

    let mut pairs = 0;
    let mut final_distance = 0 as f64;

    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        let distance = (edit_distance(a, b) / key_guess) as f64;
        final_distance += distance;
        pairs += 1;
    }

    final_distance / pairs as f64
}

pub fn edit_distance(x: &[u8], y: &[u8]) -> u8 {
    let x_bits = BitVec::from_bytes(x);
    let y_bits = BitVec::from_bytes(y);

    let mut distance: u8 = 0;
    for (a, b) in x_bits.iter().zip(y_bits) {
        if a != b {
            distance += 1
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_keysize() {
        // get the 3 smallest keysize values to try out
        let value = find_smallest_KEYSIZE();
        assert_eq!(value, [29, 25, 31]);
    }

    #[test]
    fn test_break_repeating_xor() {
        // proceeding with previous found keysize of 29
        let cipher_blocks = break_ciphertext_into_blocks(29);
        let key = solve_blocks(cipher_blocks);
        let key_as_string = str::from_utf8(&key).unwrap();
        assert_eq!(key_as_string, "Terminator X: Bring the noise");
    }

}
