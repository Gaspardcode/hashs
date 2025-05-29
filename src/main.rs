mod md5;
mod sha;
mod testsuite;
use crate::md5::*;
use crate::sha::*;

fn sample_md5() {
    let mut md5: Md5 = Md5::new();
    let mut data = String::from("The quick brown fox jumps over the lazy dog");

    md5.update(&data.as_bytes());
    data = md5.digest_string();

    println!("{}", data);
    println!("should be 9e107d9d372bb6826bd81d3542a419d6");

    md5.clear();
}

pub fn n_sha256() {
    const n: usize = 12_000_000;
    let mut sha: Sha256 = Sha256::new();
    let mut data = String::from("Hello, world!");

    for _ in 0..n {
        sha.update(&data.as_bytes());
        data = sha.digest_string();
        sha.clear();
    }
    println!("Hello, world!");
    println!("After {} sha256:", n);
    println!("{}", data);
}
fn main() {
    sample_md5();
}
