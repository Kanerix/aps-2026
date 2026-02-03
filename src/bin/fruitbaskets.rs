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
    let num_fruits: usize = lines.next().unwrap().parse().unwrap();

    let mut fruit_weights = Vec::with_capacity(num_fruits);
    let mut weights_str_list = lines.next().unwrap().split_whitespace();
    while let Some(w) = weights_str_list.next() {
        fruit_weights.push(w.parse::<usize>().unwrap());
    }

    let weights_sum = fruit_weights.iter().sum();
    let mut db_count = vec![0usize; weights_sum + 1];
    let mut db_sum_total = vec![0usize; weights_sum + 1];

    db_count[0] = 1;

    for &weight in &fruit_weights {
        for current_sum in (weight..=weights_sum).rev() {
            let target_sum = current_sum - weight;
            let prev_count = db_count[target_sum];
            if prev_count == 0 {
                continue;
            }
            let prev_sum = db_sum_total[target_sum];
            db_count[current_sum] += prev_count;
            db_sum_total[current_sum] += prev_sum + weight * prev_count;
        }
    }

    let mut answer: usize = 0;
    for s in 200..=weights_sum {
        answer += db_sum_total[s];
    }

    println!("{}", answer);
}
