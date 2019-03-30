use utils::data::CryptopalData;

fn execute() {}

#[cfg(test)]
mod tests {
    use super::CryptopalData;

    #[test]
    fn test_pkcs() {
        let challenge_string = CryptopalData::from_string("YELLOW SUBMARINE");
        let padded = challenge_string.pad(20);
        println!("{:?}", padded.to_hex());
    }
}
