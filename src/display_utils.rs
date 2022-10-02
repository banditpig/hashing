#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn dump_chunks(chunks: &Vec<Vec<u32>>) {
    for c in chunks {
        for w in c {
            println!("{}", format!("{:032b}", w));
        }
        println!();
    }
}
