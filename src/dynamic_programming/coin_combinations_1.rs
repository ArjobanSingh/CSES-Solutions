use std::io::{self, BufWriter, Read, Write};

const MOD: i32 = 1_000_000_007;

// The intuition is to find the number of ways to reach the target sum by breaking it into smaller subproblems.
// For each sum from 0 to target, iterate over each coin and update the number of ways to reach the next sums(within range)
// that can be formed by adding the current coin.
// This is done by iterating through each possible sum (from 0 to target) and adding the current sum's count
// to the counts of sums that can be formed by adding the coin.
// Formula: dp[j + coin] += dp[j] for all valid j and coins.
// Example: If coins are {2, 3, 5} and target = 5:
// - dp[5] = dp[5 - 2] + dp[5 - 3] + dp[5 - 5]
// - This means 5 can be reached as: 3 + 2, 2 + 3, or 5 alone, considering distinct combinations.
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

    let mut coins: Vec<usize> = lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // we don't need to sort it, but it helps in breaking early out of loop beneath
    // once we have reached a coin + target which is out of bound, so no point going over
    // pending coins for that sum.
    coins.sort();

    for i in 0..dp.len() {
        for coin in coins.iter() {
            if i + coin < dp.len() {
                dp[i + coin] = (dp[i + coin] + dp[i]) % MOD;
            } else {
                break;
            }
        }
    }

    writeln!(out, "{}", dp[target] % MOD).expect("error writing line");

    out.flush().ok();
}
