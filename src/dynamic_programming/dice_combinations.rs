use std::io::{self, BufWriter, Read, Write};

const MOD: i64 = 1_000_000_007;
const DICE_SIDES: usize = 6;

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

    let mut cur_sum = 1;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        // The key idea is that to determine the number of ways to reach a target sum `n` using a dice
        // with faces numbered 1 to 6, we need to sum the number of ways to reach the previous 6 sums (i.e., n-1, n-2, ..., n-6).
        // This is because any sum `n` can be reached by rolling a dice and adding the result (1 to 6) to one of the previous sums.
        // Thus, the number of ways to reach a sum `n` depends on the number of ways to reach the last 6 sums.
        if i > DICE_SIDES {
            // Handle mod for negative cases as well.
            cur_sum = (cur_sum - dp[i - (DICE_SIDES + 1)] + MOD) % MOD;
        }
        dp[i] = cur_sum;
        cur_sum = (cur_sum + dp[i]) % MOD;
    }

    writeln!(out, "{}", dp[n]).expect("error writing line");

    out.flush().ok();
}
