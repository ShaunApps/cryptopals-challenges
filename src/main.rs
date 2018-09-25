extern crate base64;
extern crate hex;

pub mod challenge_set_1;

use challenge_set_1::detect_single_char_xor::detect_sing_char_xor;
use challenge_set_1::repeating_key_xor::repeating_key_xor;
use challenge_set_1::single_byte_xor_cipher::single_byte_xor_cipher;

fn main() {
    // single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let string_phrase =
        String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");

    let string_key = String::from("ICE");

    let value = repeating_key_xor(string_phrase, string_key);
    println!("{:?}", value);
    assert_eq!(
        value,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}
