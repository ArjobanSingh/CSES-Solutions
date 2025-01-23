use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    lines.next().expect("Error reading n");
    let mut out = BufWriter::new(io::stdout().lock());

    let mut nums: Vec<i64> = lines
        .next()
        .expect("Error reading array")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();
    nums.sort();

    let mut sum: i64 = 0;
    for it in nums {
        if it > sum + 1 {
            break;
        }
        sum += it;
    }

    writeln!(out, "{}", sum + 1).expect("error writing line");

    out.flush().ok();
}
