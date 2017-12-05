# [XXTEA-Rust](https://crates.io/crates/xxtea)
[![Build Status](https://travis-ci.org/Hanaasagi/XXTEA-Rust.svg?branch=master)](https://travis-ci.org/Hanaasagi/XXTEA-Rust) ![](https://img.shields.io/badge/release-2.0-F4A466.svg)  
XXTEA encryption algorithm library  

### What is XXTEA
see this [page](https://en.wikipedia.org/wiki/XXTEA)
### Example

```Rust
extern crate xxtea;
extern crate base64;

use base64::{encode, decode};
use std::str;

fn main() {
    let data = "Hello World";
    let key = "This is the key";

    // encrypt
    let result: String = encode(xxtea::encrypt(&data, &key).as_slice());
    println!("{}", result); // will output GEvbeEorvUJmCT2A2j5bGw==

    // decrypt
    let plain_bytes: Vec<u8> = xxtea::decrypt(&decode(&result).unwrap(), &key);

    let plain_texts = match str::from_utf8(plain_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", plain_texts);  // will output Hello World
}
```

### Contributor
[Daniel](https://github.com/Digers)

### LICENSE
MIT
