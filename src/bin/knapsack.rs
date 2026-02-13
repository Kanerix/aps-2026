/// https://itu.kattis.com/courses/BAPS/APS-26/assignments/hebfvi/problems/knapsack
///
/// This is the classic 0/1 Knapsack problem.
///
/// ### Kattis description:
///
/// Implement a solution to the classic knapsack problem. You are given a
/// knapsack that can hold up to a certain weight (its capacity), and several
/// items you may choose to put in the knapsack. Each item has a weight and a
/// value. Choose a subset of the items (which could be all of them, or none of
/// them) having the greatest value that fit into the knapsack (i.e. the sum of
/// the weights of the items you choose must be less than or equal to the
/// knapsack capacity).
use std::{
    cmp::max,
    io::{self, Read},
};

#[derive(Debug)]
struct Knapsack {
    capacity: usize,
    objects: Vec<Object>,
}

#[derive(Debug)]
struct Object {
    value: usize,
    weight: usize,
}

#[derive(Debug)]
struct DP {
    knapsack: Knapsack,
    table: Vec<Vec<usize>>,
}

impl Knapsack {
    /// Create the kanpsack from lines.
    ///
    /// This panics if the lines are invalid.
    #[inline]
    fn from_lines<'a, I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let (capacity, n) = {
            let (cr, nr) = lines
                .next()
                .expect("input line is missing")
                .split_once(' ')
                .expect("input line has invalid format");

            (
                cr.parse().expect("invalid usize for capacity"),
                nr.parse().expect("invalid usize for n"),
            )
        };

        let mut objects: Vec<Object> = Vec::with_capacity(n);
        for _ in 0..n {
            objects.push(Object::from_lines(lines));
        }

        Knapsack { capacity, objects }
    }
}

impl Object {
    /// Create the knapsack item from lines.
    ///
    /// This panics if the lines are invalid.
    #[inline]
    fn from_lines<'a, I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let (value, weight) = {
            let (vr, wr) = lines
                .next()
                .expect("object line is missing")
                .split_once(' ')
                .expect("object line has invalid format");

            (
                vr.parse().expect("invalid usize for value"),
                wr.parse().expect("invalid usize for weight"),
            )
        };

        Object { value, weight }
    }
}

impl DP {
    /// Create a new dynamic programming table from a knapsack.
    fn from_knapsack(knapsack: Knapsack) -> Self {
        let object_count = knapsack.objects.len();
        let capacity = knapsack.capacity;
        let dp = vec![vec![0; capacity + 1]; object_count + 1];
        Self {
            knapsack,
            table: dp,
        }
    }

    /// Calculate all the possible knapsack solutions.
    fn fill_table(&mut self) {
        let objects_len = self.knapsack.objects.len();
        let capacity = self.knapsack.capacity;

        for i in 1..=objects_len {
            let object = &self.knapsack.objects[i - 1];

            for w in 0..=capacity {
                if object.weight > w {
                    self.table[i][w] = self.table[i - 1][w];
                } else {
                    self.table[i][w] = max(
                        self.table[i - 1][w],
                        self.table[i - 1][w - object.weight] + object.value,
                    )
                }
            }
        }
    }

    /// Backtrack to find the most valuable knapsack.
    fn chosen_indicies(&self) -> Vec<usize> {
        let mut chosen = Vec::new();
        let mut capacity = self.knapsack.capacity;
        let objects_len = self.knapsack.objects.len();

        for i in (1..=objects_len).rev() {
            if self.table[i][capacity] != self.table[i - 1][capacity] {
                let obj = &self.knapsack.objects[i - 1];
                chosen.push(i - 1);
                capacity -= obj.weight;
            }
        }

        chosen
    }
}

#[inline]
fn read_to_end() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    unsafe { String::from_utf8_unchecked(input) }
}

#[inline]
fn create_output(buf: &mut String, answer: Vec<usize>) {
    use std::fmt::Write as FmtWrite;
    writeln!(buf, "{}", answer.len()).unwrap();

    if !answer.is_empty() {
        write!(buf, "{}", answer[0]).unwrap();
        for idx in &answer[1..] {
            write!(buf, " {}", idx).unwrap();
        }
    }
    writeln!(buf).unwrap();
}

fn main() {
    let input = read_to_end();
    let mut lines = input.lines().peekable();

    let mut knapsacks: Vec<Knapsack> = Vec::new();
    while lines.peek().is_some() {
        let knapsack = Knapsack::from_lines(&mut lines);
        knapsacks.push(knapsack);
    }

    let mut output = String::new();
    for knapsack in knapsacks {
        let mut db = DP::from_knapsack(knapsack);
        db.fill_table();
        for r in &db.table {
            println!("{:?}", r);
        }
        let answer = db.chosen_indicies();
        create_output(&mut output, answer);
    }

    print!("{output}");
}
