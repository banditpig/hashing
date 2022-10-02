//See https://en.wikipedia.org/wiki/SHA-2
mod chunk_utils;
mod display_utils;
mod sha_algorithms;
mod word_utils;

use crate::chunk_utils::{find_num, make_16x32_blocks, string_to_bytes};
use crate::sha_algorithms::sha256;
use crate::word_utils::{ch, maj, s0, s1, K, S0, S1};

fn main() {
    let hash = sha256("".to_string());
    print!("{}", hash);
}
