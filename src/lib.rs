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


fn to_u32(bytes: &Vec<u8>, include_length: bool) -> Vec<u32> {
    let length: u32 = bytes.len() as u32;
    let mut n: u32 = length >> 2;
    if length&3 != 0 {
        n = n + 1;    
    }
    let mut v;
    if include_length {
        v = vec![0; n as usize + 1];
        v[n as usize] = length;    
    } else {
        v = vec![0; n as usize];    
    }
    for i in 0..length {
        v[(i>>2) as usize] = (bytes[i as usize] as u32)<< ((i & 3) << 3) as u32;
    }
    return v
}
