/// https://itu.kattis.com/courses/BAPS/APS-26/assignments/jgeu62/problems/geppetto

use std::{
    fmt::Debug,
    io::{self, Read},
    str::FromStr,
};

#[inline]
fn parse<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.trim().parse().unwrap()
}

#[inline]
fn read_to_end() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    unsafe { String::from_utf8_unchecked(input) }
}

fn main() {
    let input = read_to_end();
    let mut lines = input.lines();
    let (n, m) = lines.next().unwrap().split_once(' ').unwrap();
    let n = parse::<usize>(n);
    let m = parse::<usize>(m);

    let mut bad: Vec<(usize, usize)> = Vec::with_capacity(m);
    while let Some(line) = lines.next() {
        let (a, b) = line.split_once(' ').unwrap();
        let a: usize = parse(a);
        let b: usize = parse(b);
        bad.push((a, b));
    }


    let subsets = 1usize << n;
    let mut ans: i64 = 0;

    for mask in 0..subsets {
        let mut ok = true;
        for (a, b) in bad.iter() {
            let has_a = (mask & (1usize << (a - 1))) != 0;
            let has_b = (mask & (1usize << (b - 1))) != 0;
            if has_a && has_b {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
