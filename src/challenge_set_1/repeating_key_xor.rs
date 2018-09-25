use hex;
use std::str::from_utf8;

pub fn repeating_key_xor(item: String, key: String) -> String {
    let mut xor_vec = Vec::new();
    let mut index = 0;

    for i in item.into_bytes().iter() {
        let xored = key.as_bytes()[index] ^ i;

        if ((index + 1) == key.as_bytes().len()) {
            index = 0;
        } else {
            index += 1;
        }
        xor_vec.push(xored);
    }

    let result = hex::encode(xor_vec);
    result
}
