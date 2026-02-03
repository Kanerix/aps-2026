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

    let mut weights = Vec::with_capacity(n);
    let mut weights_raw = lines.next().unwrap().split_whitespace();
    while let Some(w) = weights_raw.next() {
        weights.push(w.parse::<usize>().unwrap());
    }

    let total_sum = weights.iter().sum();
    let max_sum = total_sum;

    let mut count = vec![0usize; max_sum + 1];
    let mut sum = vec![0usize; max_sum + 1];

    count[0] = 1;

    for &w in &weights {
        for s in (w..=max_sum).rev() {
            let target_sum = s - w;
            let prev_count = count[target_sum];
            if prev_count == 0 {
                continue;
            }
            let prev_sum = sum[target_sum];
            count[s] += prev_count;
            sum[s] += prev_sum + w * prev_count;
        }
    }

    let mut answer: usize = 0;
    for s in 200..=max_sum {
        answer += sum[s];
    }

    println!("{}", answer);
}
