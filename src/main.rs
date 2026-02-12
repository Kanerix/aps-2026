use std::io::{self, Read};

#[inline]
fn read_to_end() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    unsafe { String::from_utf8_unchecked(input) }
}

fn main() {
    let input = read_to_end();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..n {
    }
}
