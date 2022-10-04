#![allow(non_snake_case)]
use crate::{ch, find_num, maj, make_16x32_blocks, s0, s1, string_to_bytes, K, S0, S1};

fn create_chunks(s: String) -> Vec<Vec<u32>> {
    //bytes from the message

    let mut start_bytes = string_to_bytes(s);
    //empty string -> e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let original_length = start_bytes.len() * 8;
    //extra 1000 0000 at the end of the message bytes
    start_bytes.push(128_u8);

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
    make_16x32_blocks(&start_bytes)
}
fn init_w(chunk: &[u32]) -> Vec<u32> {
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

    for chunk in &chunks {
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
        let x = &format!("{:08x}", u);
        res += x;
    }

    res
}
#[cfg(test)]
mod tests {
    use crate::sha_algorithms::sha256;
    // use crate::{s1, sha256};

    #[test]
    pub fn sha256_test() {
        let mut s = "".to_string();
        assert_eq!(
            sha256(s),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        //
        s = "hello world".to_string();
        assert_eq!(
            sha256(s),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );

        s = "!@£$%%^&**()-=".to_string();
        assert_eq!(
            sha256(s),
            "842bef3825efe57acf0192f04b78be9d859ae977f6a4ab5eb17c63fc0dedabcd"
        );

        s = "Aenean quis lobortis arcu, eleifend consectetur urna. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Suspendisse feugiat sed ligula finibus ornare. Donec dignissim sollicitudin magna, at suscipit quam scelerisque nec. Suspendisse lacinia velit non varius sollicitudin. Nullam eget semper nunc. Quisque vitae quam erat. Cras nec purus pulvinar, semper nibh sed, imperdiet arcu. Mauris mattis nisi dictum, vestibulum mauris nec, imperdiet libero. Praesent vitae lorem sollicitudin, vestibulum ante a, faucibus odio. Nam id convallis magna. Fusce sapien mauris, dapibus nec velit vel, efficitur tempor eros. Nunc et massa condimentum, vehicula neque eget, condimentum orci. Morbi pretium cursus lorem, a interdum arcu eleifend eu. Donec id risus dolor.".to_string();
        assert_eq!(
            sha256(s),
            "af7e27521dea753fdf274cb66e81293d45edf9c3f18eef43beb3b32aafecd802"
        );
    }
}
//
