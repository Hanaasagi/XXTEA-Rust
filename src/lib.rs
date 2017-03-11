const DELTA: u32 = 0x9E3779B9;

fn to_bytes(v: &Vec<u32>, include_length: bool) -> Vec<u8> {
    let length: u32 = v.len() as u32;
    let mut n: u32 = length << 2;
    if include_length {
        let m = v[length as usize - 1];
        n = n - 4;
        assert!(((m < n-3) || (m > n)));
        n = m;
    }
    let mut bytes: Vec<u8> = vec![0; n as usize];
    for i in 0..n {
        bytes[i as usize] = (v[(i >> 2) as usize] >> ((i & 3) << 3)) as u8;
    }
    return bytes;
}

