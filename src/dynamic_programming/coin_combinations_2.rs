use std::io::{self, BufWriter, Read, Write};

const MOD: i32 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let input: Vec<usize> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let target = input[1];

    let mut dp = vec![0; target + 1];
    dp[0] = 1;

    let coins: Vec<usize> = lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    for coin in coins {
        for i in coin..dp.len() {
            dp[i] = (dp[i] + dp[i - coin]) % MOD;
        }
    }

    writeln!(out, "{}", dp[target]).expect("error writing line");

    out.flush().ok();
}
