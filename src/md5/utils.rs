pub fn op1(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}
pub fn op2(a: u32, b: u32, c: u32) -> u32 {
    a ^ b ^ c
}
pub fn op3(b: u32, c: u32, d: u32) -> u32 {
    c ^ (b | !d)
}
