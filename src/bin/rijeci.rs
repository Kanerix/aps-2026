/// https://itu.kattis.com/courses/BAPS/APS-26/assignments/hebfvi/problems/rijeci

use std::io::{self, Read};

#[inline]
fn read_to_end() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    unsafe { String::from_utf8_unchecked(input) }
}

fn main() {
    let input = read_to_end();
    let n: usize = input.trim().parse().unwrap();

    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 0..n - 1 {
        let tmp = n2;
        n2 = n1 + n2;
        n1 = tmp;
    }

    println!("{n1} {n2}")
}
