use std::{
    collections::VecDeque,
    io::{self, Read},
};

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

    bags.sort();

    let mut bags = VecDeque::from(bags);

    let mut trips = 0;

    while bags.len() != 0 {
        if bags.len() == 1 {
            trips += 1;
            break;
        }

        let lightest = bags.pop_front();
        let heaviest = bags.pop_back();

        match (lightest, heaviest) {
            (Some(l), Some(h)) => {
                if l + h > m {
                    trips += 1;
                    bags.push_front(l);
                } else {
                    trips += 1;
                }
            }
            _ => {
                trips += 1;
            }
        }
    }

    println!("{}", trips);
}
