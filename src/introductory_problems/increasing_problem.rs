use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    lines.next().expect("Error reading n");

    let mut vec: Vec<i32> = lines
        .next()
        .expect("Error reading array")
        .split_ascii_whitespace()
        .filter_map(|val| val.parse().ok())
        .collect();

    let mut ans: i64 = 0;
    for idx in 1..vec.len() {
        let prev = idx - 1;
        let diff = vec[idx] - vec[prev];
        if diff >= 0 {
            continue;
        }

        ans += diff.abs() as i64;
        vec[idx] = vec[prev]; // update item at this position
    }
    writeln!(out, "{ans}").expect("Error writing ans");

    out.flush().ok();
}
