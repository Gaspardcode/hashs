mod testsuite;
mod sha;
use crate::sha::*;
fn main() {
    println!("Hello, world!");
    let data = "helloooo";
    let mut sha:Sha1 = Sha1::new();
    sha.update(&data.as_bytes());
    let result = sha.digest_string();
}
