use base64;
use hex;
use std::ops::BitXor;

// Fixed XOR
#[derive(Debug, PartialEq)]
struct BytesXOR(Vec<u8>);

impl BitXor for BytesXOR {
    type Output = Self;

    fn bitxor(self, BytesXOR(rhs): Self) -> Self {
        let BytesXOR(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        BytesXOR(lhs.iter().zip(rhs.iter()).map(|(x, y)| *x ^ *y).collect())
    }
}

pub fn fixed_XOR(data_1: &str, data_2: &str) -> String {
    let one = hex::decode(data_1).unwrap();
    let two = hex::decode(data_2).unwrap();
    let xor = BytesXOR(one) ^ BytesXOR(two);
    hex::encode(xor.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_XOR() {
        let data_1 = "1c0111001f010100061a024b53535009181c";
        let data_2 = "686974207468652062756c6c277320657965";
        assert_eq!(
            fixed_XOR(data_1, data_2),
            "746865206b696420646f6e277420706c6179"
        );
    }
}
