use base64;
use hex;

// Convert hex to base64
pub fn convert_hex_to_64(data: &str) -> String {
    let data_as_bytes = hex::decode(data).unwrap();
    let b64 = base64::encode(&data_as_bytes);
    b64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_64() {
        let string_data = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            convert_hex_to_64(string_data),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

}
