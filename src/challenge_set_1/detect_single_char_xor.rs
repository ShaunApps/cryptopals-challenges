use challenge_set_1::single_byte_xor_cipher::single_byte_xor_cipher;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn detect_sing_char_xor() {
    let f = File::open("1_challenge_4.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        single_byte_xor_cipher(&l);
    }
}
