extern crate rand;
extern crate xxtea;

use std::str;
use rand::Rng;

fn rand_string() -> String {
    let num = rand::thread_rng().gen_range(1, 256);
    (0..num).map(|_| rand::random::<char>()).collect()
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
fn test() {
    for _ in 0..100 {
        run_test_case();
    }
}
