use base64;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use rand::prelude::*;
use rand::{OsRng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

// AES in ECB mode
fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize256, key, blockmodes::NoPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes() {
        let mut f = File::open("1_challenge_7.txt").unwrap();
        let mut buf_reader = BufReader::new(f);

        let mut buf = Vec::new();

        for line in buf_reader.lines() {
            buf.extend(line.unwrap().into_bytes().iter().cloned())
        }

        let cipher = base64::decode(&buf).unwrap();

        let key = b"YELLOW SUBMARINE";

        let mut iv: [u8; 8] = [0; 8];
        let mut rng = OsRng::new().ok().unwrap();
        rng.fill_bytes(&mut iv);

        let decrypted_data = decrypt(&cipher, key).ok().unwrap();
        println!("{:?}", String::from_utf8(decrypted_data).unwrap());
    }

}
