extern crate base64;
extern crate hex;

pub mod challenge_set_1;

use challenge_set_1::detect_single_char_xor::detect_sing_char_xor;
use challenge_set_1::single_byte_xor_cipher::single_byte_xor_cipher;

fn main() {
    // single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    detect_sing_char_xor();
}
