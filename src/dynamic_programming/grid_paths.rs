use std::{
    io::{self, BufWriter, Read, Write},
    usize,
};

const MOD: i32 = 1_000_000_007;
const TRAP: char = '*';

fn get_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let n: usize = lines
        .next()
        .expect("Error reading n")
        .parse()
        .expect("Error parsing n");

    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(n);

    for line in lines {
        matrix.push(line.chars().collect());
    }

    let mut dp: Vec<i32> = vec![0; n * n];
    dp[0] = if matrix[0][0] == TRAP { 0 } else { 1 };

    for r in 0..n {
        for c in 0..n {
            if matrix[r][c] == TRAP {
                continue;
            }

            let cur_idx = get_idx(r, c, n);
            if r > 0 {
                dp[cur_idx] = (dp[cur_idx] + dp[get_idx(r - 1, c, n)]) % MOD;
            }

            if c > 0 {
                dp[cur_idx] = (dp[cur_idx] + dp[get_idx(r, c - 1, n)]) % MOD;
            }
        }
    }

    writeln!(out, "{}", dp[n * n - 1]).expect("error writing line");

    out.flush().ok();
}
