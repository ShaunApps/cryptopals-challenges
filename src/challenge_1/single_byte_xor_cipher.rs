use base64;
use hex;

// this isn't a great implementation, need to refractor
// with a word frequency counter or something

// Convert hex to base64
pub fn single_byte_xor_cipher(data: &str) {
    let data_as_bytes = hex::decode(data).unwrap();
    let length = data_as_bytes.len();
    for byte in 0..=255 {
        let byte_vector = vec![byte; length];
        let xor = String::from_utf8(
            data_as_bytes
                .iter()
                .zip(byte_vector.iter())
                .map(|(x, y)| *x ^ *y)
                .collect(),
        ).unwrap();
        if xor == "Cooking MC's like a pound of bacon" {
            println!("Xor byte: {}", byte)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_xor_cipher() {}

}
