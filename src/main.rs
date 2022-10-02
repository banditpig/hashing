use modular::Modular;
use std::ops::Shr;

//See https://en.wikipedia.org/wiki/SHA-2

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn rotl(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

pub fn rotr(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}
pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}
pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

pub fn S0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

pub fn S1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

pub fn s0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ (x.shr(3))
}

pub fn s1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ (x.shr(10))
}

pub fn dump(v: &Vec<u8>) {
    let mut iy = 0;
    while iy < v.len() {
        for ix in 0..4 {
            print!("{}  ", format!("{:08b}", v[iy + ix]));
        }
        iy += 4;
        println!();
    }
}
pub fn dump_chunks(chunks: &Vec<Vec<u32>>) {
    for c in chunks {
        for w in c {
            println!("{}", format!("{:032b}", w));
        }
        println!();
    }
}

pub fn find_num(n: u32, k: u32) -> u32 {
    let rem = ((n + k) as i32).to_modulo(k).remainder() as u32;
    return if rem == 0 { 0 } else { k - rem };
}
pub fn string_to_bytes(s: String) -> Vec<u8> {
    s.as_bytes().to_vec()
}
pub fn convert_4_u8_to_u32(arr: &[u8]) -> u32 {
    let a = (arr[0] as u32) << 24;
    let b = (arr[1] as u32) << 16;
    let c = (arr[2] as u32) << 8;
    let d = (arr[3] as u32) << 0;
    a + b + c + d
}
pub fn make_16x32_blocks(bytes: &Vec<u8>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for block64 in bytes.chunks(64) {
        let mut v: Vec<u32> = vec![];
        for block4 in block64.chunks(4) {
            let w = convert_4_u8_to_u32(block4);
            v.push(w);
        }
        result.push(v);
    }
    result
}

pub fn hash(chunks: &Vec<Vec<u32>>) -> String {
    let mut H0: u32 = 0x6a09e667;
    let mut H1: u32 = 0xbb67ae85;
    let mut H2: u32 = 0x3c6ef372;
    let mut H3: u32 = 0xa54ff53a;
    let mut H4: u32 = 0x510e527f;
    let mut H5: u32 = 0x9b05688c;
    let mut H6: u32 = 0x1f83d9ab;
    let mut H7: u32 = 0x5be0cd19;

    for i in 0..chunks.len() {
        let mut w: Vec<u32> = vec![];
        let chunk = &chunks[i];

        for t in 0..64 {
            if t <= 15 {
                w.push(chunk[t]);
            } else {
                let s0 = s0(w[t - 15]);
                let s1 = s1(w[t - 2]);
                let chnk = w[t - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[t - 7])
                    .wrapping_add(s1);

                w.push(chnk);
            }
        }

        let mut a = H0;
        let mut b = H1;
        let mut c = H2;
        let mut d = H3;
        let mut e = H4;
        let mut f = H5;
        let mut g = H6;
        let mut h = H7;

        for t in 0..64 {
            let S1 = S1(e);
            let ch = ch(e, f, g);
            let temp1 = h
                .wrapping_add(S1)
                .wrapping_add(ch)
                .wrapping_add(K[t])
                .wrapping_add(w[t]);

            let S0 = S0(a);
            let maj = maj(a, b, c);
            let temp2 = S0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        H0 = a.wrapping_add(H0);
        H1 = b.wrapping_add(H1);
        H2 = c.wrapping_add(H2);
        H3 = d.wrapping_add(H3);
        H4 = e.wrapping_add(H4);
        H5 = f.wrapping_add(H5);
        H6 = g.wrapping_add(H6);
        H7 = h.wrapping_add(H7);
    }
    let mut res = String::new();
    for u in [H0, H1, H2, H3, H4, H5, H6, H7] {
        let x = &format!("{:x}", u);
        res += x;
    }
    res
}

fn main() {
    //bytes from the message
    let mut start_bytes = string_to_bytes("hi".to_string());

    let original_length = start_bytes.len() * 8;
    //extra 1000 0000 at the end of the message bytes
    start_bytes.push(128 as u8);

    //find padding size
    let current_len = 8 * start_bytes.len();
    let extra = find_num((current_len + 64) as u32, 512);
    //2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
    //2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
    //pad out
    for _ix in 0..(extra / 8) {
        start_bytes.push(0u8);
    }
    //original length as 64 bits
    start_bytes.append(&mut (original_length as u64).to_be_bytes().to_vec());

    //512 bit blocks
    let chunks = make_16x32_blocks(&start_bytes);

    //Now do the hash
    let hash = hash(&chunks);
    print!("{}", hash);
}
