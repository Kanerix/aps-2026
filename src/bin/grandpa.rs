/// https://itu.kattis.com/courses/BAPS/APS-26/assignments/guzmvm/problems/grandpabernie

use std::{collections::HashMap, io::stdin};

fn main() {
    let mut countries: HashMap<String, Vec<u32>> = HashMap::new();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut line_buf = String::new();
        stdin().read_line(&mut line_buf).unwrap();

        if let Some((country, year_str)) = line_buf.trim().split_once(' ') {
            let year = year_str.parse::<u32>().unwrap();
            let list = countries.entry(country.to_string()).or_insert(Vec::new());
            list.push(year);
        }
    }

    for (_, v) in countries.iter_mut() {
        v.sort()
    }

    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let x = buf.trim().parse::<usize>().unwrap();

    for _ in 0..x {
        let mut line_buf = String::new();
        stdin().read_line(&mut line_buf).unwrap();

        if let Some((country, year_str)) = line_buf.trim().split_once(' ') {
            let trip = year_str.parse::<usize>().unwrap();
            if let Some(list) = countries.get(country) {
                println!("{:?}", list[trip - 1]);
            }
        }
    }
}
