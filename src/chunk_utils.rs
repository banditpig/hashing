use modular::Modular;
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
