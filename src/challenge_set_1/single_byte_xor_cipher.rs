use base64;
use hex;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn rank_char_frequency(data: &Vec<u8>) -> i32 {
    // i took this from https://laconicwolf.com/2018/05/29/cryptopals-challenge-3-single-byte-xor-cipher-in-python/
    // this is a HashMap of the character frequency in english
    let character_frequency: HashMap<&str, i32> = [
        ("a", 8167),
        ("b", 1492),
        ("c", 2782),
        ("d", 4253),
        ("e", 12702),
        ("f", 2228),
        ("g", 2015),
        ("h", 6094),
        ("i", 6094),
        ("j", 153),
        ("k", 772),
        ("l", 4025),
        ("m", 2406),
        ("n", 6749),
        ("o", 7507),
        ("p", 1929),
        ("q", 95),
        ("r", 5987),
        ("s", 6327),
        ("t", 9056),
        ("u", 2758),
        ("v", 978),
        ("w", 2360),
        ("x", 150),
        ("y", 1974),
        ("z", 74),
        (" ", 13000),
    ].iter()
        .cloned()
        .collect();
    let mut score: i32 = 0;
    let message = from_utf8(data);
    let message = match message {
        Ok(m) => m.to_lowercase(),
        Err(e) => String::from(""),
    };
    for l in message.chars() {
        if let Some(i) = character_frequency.get::<str>(&l.to_string()) {
            score += i;
        }
    }

    score
}

pub fn single_byte_xor_cipher(data: &str) -> String {
    let data_as_bytes = hex::decode(data).unwrap();
    let length = data_as_bytes.len();
    let mut top_score: i32 = 0;
    let mut phrase: String = "".to_string();
    for byte in 0..=255 {
        let xored: Vec<_> = data_as_bytes.iter().map(|&x| x ^ byte).collect();
        // here is a second way to XOR it
        // let byte_vector = vec![byte; length];
        // let xor = String::from_utf8(
        //     data_as_bytes
        //         .iter()
        //         .zip(byte_vector.iter())
        //         .map(|(x, y)| *x ^ *y)
        //         .collect(),
        // ).unwrap();
        let char_score = rank_char_frequency(&xored);
        // println!("char_score: {:?}", char_score);
        if char_score > top_score {
            top_score = char_score;
            let thing = String::from_utf8(xored);
            phrase = match thing {
                Ok(x) => x,
                Err(error) => String::from(""),
            };
        }
    }
    println!("{:?}", phrase);
    phrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_xor_cipher() {}

}
