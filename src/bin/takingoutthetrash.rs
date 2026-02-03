use std::io::{self, Read};

#[inline]
fn read_to_end() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    unsafe { String::from_utf8_unchecked(input) }
}

fn main() {
    let input = read_to_end();
    let mut args = input.split_whitespace();
    let n: usize = args.next().unwrap().parse().unwrap();
    let m: usize = args.next().unwrap().parse().unwrap();

    let mut bags: Vec<usize> = Vec::with_capacity(n);
    while let Some(w) = args.next() {
        bags.push(w.parse().unwrap())
    }

    if n == 0 {
        println!("0");
        return;
    }

    bags.sort();

    let mut i = 0;
    let mut j = n - 1;
    let mut trips = 0;

    while i <= j {
        if bags[i] + bags[j] <= m {
            i += 1;
        }

        if j == 0 {
            trips += 1;
            break;
        }

        j -= 1;
        trips += 1
    }

    println!("{}", trips);
}
