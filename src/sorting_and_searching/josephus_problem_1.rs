use std::io::{self, BufWriter, Read, Write};

// TC: O(N lgN). Log base: 2
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let n: i32 = lines
        .next()
        .expect("Error reading n")
        .parse()
        .expect("error parsing n");

    let mut out = BufWriter::new(io::stdout().lock());
    let mut nums: Vec<i32> = (1..=n).collect();
    let mut added = 0;

    let mut print = false;
    while added != n {
        for idx in 0..n {
            let num = nums[idx as usize];
            if num == -1 {
                continue;
            }

            if print {
                write!(out, "{num} ").expect("error writing num");
                nums[idx as usize] = -1;
                added += 1;
            }

            print = !print;
        }
    }

    writeln!(out, "").expect("error writing line");

    out.flush().ok();
}
