# XXTEA-Rust
[XXTEA](https://en.wikipedia.org/wiki/XXTEA) encryption algorithm library  

###Usage

    extern crate xxtea;

    fn main() {
        let data = "Hello World";
        let key = "This is the key";
        
        // encrypt
        let result: Vec<u8> = xxtea::encrypt(&data, &key);

        // decrypt
        let plain_bytes: Vec<u8> = xxtea::decrypt(&result, &key);
    }
