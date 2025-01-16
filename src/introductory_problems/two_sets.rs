use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let n: i64 = lines
        .next()
        .expect("Error reading array")
        .trim()
        .parse()
        .expect("Error parsing n");

    let sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        // sum is odd, not possible
        writeln!(out, "NO").expect("Error writing ans");
        return;
    }

    let mut half = sum / 2;
    let mut first = Vec::new();
    let mut second = Vec::new();

    for i in (1..=n).rev() {
        if half >= i {
            first.push(i);
            half -= i;
        } else {
            second.push(i);
        }
    }

    writeln!(out, "YES").expect("Error writing ans");
    writeln!(out, "{}", first.len()).expect("Error writing ans");

    for it in first {
        write!(out, "{it} ").expect("Error writing ans");
    }

    writeln!(out, "").expect("Error writing ans");
    writeln!(out, "{}", second.len()).expect("Error writing ans");
    for it in second {
        write!(out, "{it} ").expect("Error writing ans");
    }
    writeln!(out, "").expect("Error writing ans");

    out.flush().ok();
}
