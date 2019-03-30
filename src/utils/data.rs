use rustc_serialize::hex::ToHex;
use std::char::from_u32;
use std::fmt;

#[derive(Debug)]
pub struct CryptopalData {
    data: Vec<u8>,
}

impl fmt::Display for CryptopalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.data.to_hex())
    }
}

impl CryptopalData {
    pub fn from_string(s: &str) -> CryptopalData {
        let data = s.as_bytes().to_vec();
        CryptopalData { data: data }
    }

    pub fn pad(&self, length: usize) -> CryptopalData {
        let mut data = self.data.clone();
        let p = length - data.len() % length;
        let p_char = from_u32(p as u32).unwrap();
        for _ in 0..p {
            data.push(p_char as u8);
        }
        CryptopalData { data: data }
    }

    pub fn to_hex(&self) -> String {
        self.data.as_slice().to_hex()
    }
}

#[cfg(test)]
mod tests {
    use super::CryptopalData;

    #[test]
    fn sanity_check() {
        let data = CryptopalData::from_string("YELLOW SUBMARINE");
    }

}
