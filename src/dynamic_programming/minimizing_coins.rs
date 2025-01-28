use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

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

    let x = input[1];

    let mut dp = vec![i32::MAX; x + 1];
    dp[0] = 0;

    // The idea is to track the minimum number of coins needed to form sums from 0 to X (target).
    // Initially, set dp[0] = 0 (0 coins for sum 0) and dp[i] = infinity for all other sums.
    // For each coin, iterate through sums from coin's value up to the target sum X.
    // Update dp[j] as the minimum of its current value or dp[j - coin] + 1.
    // This means if a sum j can be formed by adding this coin to a previous sum (j - coin),
    // we update dp[j] to reflect the new minimum number of coins needed to reach sum j.
    for coin in lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
    {
        if let Ok(coin) = coin.parse::<usize>() {
            for i in coin..=x {
                dp[i] = cmp::min(
                    dp[i],
                    if dp[i - coin] == i32::MAX {
                        i32::MAX
                    } else {
                        dp[i - coin] + 1
                    },
                );
            }
        }
    }

    writeln!(out, "{}", if dp[x] == i32::MAX { -1 } else { dp[x] }).expect("error writing line");

    out.flush().ok();
}
