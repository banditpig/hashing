//See https://en.wikipedia.org/wiki/SHA-2
mod chunk_utils;
mod display_utils;
mod sha_algorithms;
mod word_utils;
use std::fs;

use crate::chunk_utils::{find_num, make_16x32_blocks, string_to_bytes};
use crate::sha_algorithms::sha256;
use crate::word_utils::{ch, maj, s0, s1, K, S0, S1};
use clap::Parser;
use std::error::Error;
use std::fs::read;

/// Utility to calculate hashes.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name and path of file to digest
    #[arg(short, long)]
    file_name: String,
}

fn read_file(f: &str) -> Result<String, Box<dyn Error>> {
    let txt = fs::read_to_string(f)?;

    Ok(txt)
}
fn main() {
    let args = Args::parse();

    match read_file(&args.file_name) {
        Ok(txt) => {
            let sh = sha256(txt);
            println!("{}", sh);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
