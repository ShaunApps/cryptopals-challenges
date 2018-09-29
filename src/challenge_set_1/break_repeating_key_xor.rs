use challenge_set_1::bit_vec::BitVec;

pub fn edit_distance(x: &str, y: &str) -> usize {
    let x_bits = BitVec::from_bytes(x.as_bytes());
    let y_bits = BitVec::from_bytes(y.as_bytes());

    let mut distance: usize = 0;
    for (a, b) in x_bits.iter().zip(y_bits) {
        if a != b {
            distance += 1
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        let x = String::from("this is a test");
        let y = String::from("wokka wokka!!!");
        assert_eq!(edit_distance(&x, &y), 37);
    }

}
