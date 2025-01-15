use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    let mut n: i64 = lines
        .next()
        .expect("Error reading t")
        .trim()
        .parse()
        .expect("Error in parsing t");

    while n > 1 {
        write!(out, "{n} ").expect("Error writing ans");

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    write!(out, "{n} ").expect("Error writing ans");
    writeln!(out, "").expect("Error writing ans");

    out.flush().ok();
}
