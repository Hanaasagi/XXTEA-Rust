extern crate rand;
extern crate xxtea;

use std::str;
use rand::Rng;

fn rand_string() -> String {
    let num = rand::thread_rng().gen_range(1, 256);
    (0..num).map(|_| rand::random::<char>()).collect()
}

pub fn to_hex_string(bytes: &[u8] ) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();

    strs.join(" ")
}

fn run_test_case() {
    let data = rand_string();
    let key = rand_string();

    let result: Vec<u8> = xxtea::encrypt(&data, &key);

    let plain_bytes: Vec<u8> = xxtea::decrypt(&result, &key);
    let plain_text = match str::from_utf8(plain_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    assert_eq!(data, plain_text);
}

#[test]
fn run_raw_test_case() {
    let set1_unaligned : [u8; 10] = [17,  18, 19,  20, 0,  0,   0,  0,   52,  238];
    let set1_encrypted : [u8; 12] = [153, 30, 118, 66, 15, 149, 77, 188, 228, 138, 105, 92];
    let set1_aligned : [u8; 12]   = [17,  18, 19,  20, 0,  0,   0,  0,   52,  238, 0,   0];

    let key = "Snakeoil";

    let result1: Vec<u8> = xxtea::encrypt_raw(&set1_unaligned.to_vec(), &key);
    let result2: Vec<u8> = xxtea::encrypt_raw(&set1_aligned.to_vec(), &key);

    assert_eq!(result1, result2);
    assert_eq!(result1, set1_encrypted.to_vec());

    // Check that encrypted length is same as input length

    let plain_bytes: Vec<u8> = xxtea::decrypt_raw(&result1, &key);
    assert_eq!(plain_bytes, set1_aligned.to_vec());
}

#[test]
fn test() {
    for _ in 0..100 {
        run_test_case();
    }
}
