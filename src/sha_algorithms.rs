#![allow(non_snake_case)]
use crate::{ch, find_num, maj, make_16x32_blocks, s0, s1, string_to_bytes, K, S0, S1};

fn create_chunks(s: String) -> Vec<Vec<u32>> {
    //bytes from the message
    let mut start_bytes = string_to_bytes(s);
    //empty string -> e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let original_length = start_bytes.len() * 8;
    //extra 1000 0000 at the end of the message bytes
    start_bytes.push(128 as u8);

    //find padding size
    let current_len = 8 * start_bytes.len();
    let extra = find_num((current_len + 64) as u32, 512);
    //pad out
    for _ix in 0..(extra / 8) {
        start_bytes.push(0u8);
    }
    //original length as 64 bits
    start_bytes.append(&mut (original_length as u64).to_be_bytes().to_vec());

    //512 bit blocks
    let chunks = make_16x32_blocks(&start_bytes);
    chunks
}
fn init_w(chunk: &Vec<u32>) -> Vec<u32> {
    let mut w: Vec<u32> = vec![];
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
    w
}

pub fn sha256(s: String) -> String {
    let chunks = create_chunks(s);
    let mut H0: u32 = 0x6a09e667;
    let mut H1: u32 = 0xbb67ae85;
    let mut H2: u32 = 0x3c6ef372;
    let mut H3: u32 = 0xa54ff53a;
    let mut H4: u32 = 0x510e527f;
    let mut H5: u32 = 0x9b05688c;
    let mut H6: u32 = 0x1f83d9ab;
    let mut H7: u32 = 0x5be0cd19;

    for i in 0..chunks.len() {
        let chunk = &chunks[i];

        let w = init_w(chunk);

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
