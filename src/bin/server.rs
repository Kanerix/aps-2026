use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();

    let input = lines.next().unwrap().split_once(' ');
    let (_, tt_str) = input.unwrap();
    let tt: u32 = tt_str.parse().unwrap();

    let tasks = lines.next().unwrap().split(' ');
    let mut tasks_done = 0;
    let mut time_used = 0;
    for time_str in tasks {
        let time: u32 = time_str.parse().unwrap();
        let time_when_completed = time_used + time;
        if time_when_completed <= tt {
            time_used += time;
            tasks_done += 1;
        } else {
            break;
        }
    }

    println!("{}", tasks_done);
}
