use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

fn largest_digit(mut num: i32) -> i32 {
    let mut digit = 0;
    while num != 0 {
        digit = cmp::max(digit, num % 10);
        num = num / 10
    }
    digit
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let mut n: i32 = lines
        .next()
        .expect("Error reading n")
        .parse()
        .expect("error parsing n");

    let mut result = 0;
    while n != 0 {
        let largest = largest_digit(n);
        n -= largest;
        result += 1;
    }

    writeln!(out, "{}", result).expect("error writing line");

    out.flush().ok();
}
