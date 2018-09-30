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
// may need to mod the hex 64 part here
fn break_ciphertext_into_blocks(keysize: u8) -> Vec<Vec<u8>> {
    let mut f = File::open("1_challenge_6.txt").unwrap();
    let mut block_vec = Vec::new();
    let mut done = false;
    while !done {
        let block = read_n(&mut f, keysize);
        match block {
            Ok(block) => block_vec.push(block),
            Err(e) => done = true,
        }
    }
    let mut trans_vec = Vec::new();
    for i in 0..keysize {
        let mut level_vec = Vec::new();
        block_vec.iter().map(|x| {
            level_vec.push(x[i as usize]);
        });
        trans_vec.push(level_vec)
    }
    trans_vec
}

fn find_smallest_KEYSIZE() -> u8 {
    let mut key_map: HashMap<u8, u8> = HashMap::new();
    for i in 1..=40 {
        let key_size = normalized_KEYSIZE(i);
        key_map.insert(key_size, i);
    }
    let mut keys = Vec::new();
    for (key, value) in key_map.iter() {
        keys.push(key);
    }
    keys.sort();
    let value = key_map.get(keys[0]).unwrap().to_owned();
    value
}

fn normalized_KEYSIZE(key_guess: u8) -> u8 {
    let mut f = File::open("1_challenge_6.txt").unwrap();
    let one = read_n(&mut f, key_guess).unwrap();
    let two = read_n(&mut f, key_guess).unwrap();
    let norm_keysize = edit_distance(&one, &two) / key_guess;
    norm_keysize
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

fn read_n<R>(reader: R, bytes_to_read: u8) -> Result<Vec<u8>, io::Error>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read as u64);
    // Do appropriate error handling for your situation
    let n = match chunk.read_to_end(&mut buf) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    assert_eq!(bytes_to_read as usize, n);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_edit_distance() {
    //     let x = String::from("this is a test");
    //     let y = String::from("wokka wokka!!!");
    //     assert_eq!(edit_distance(&x, &y), 37);
    // }

    #[test]
    fn test_find_smallest_keysize() {
        let value = find_smallest_KEYSIZE();
        assert_eq!(value, 5);
    }

    #[test]
    fn test_key_test() {
        let small_guess = find_smallest_KEYSIZE();
        let cipher_blocks = break_ciphertext_into_blocks(small_guess);
        let key = solve_blocks(cipher_blocks);
        let test_against = vec![5, 6, 7];
        assert_eq!(key, test_against);
    }

}
