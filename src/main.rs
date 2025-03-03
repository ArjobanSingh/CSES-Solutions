use std::io::{self, BufWriter, Read, Write};

// Space Optimization:
// Instead of using a 2D DP table, we only maintain two rows at a time:
// - The previous row (`dp`), which represents `dp[i-1][j]` values
// - The current row (`new_dp`), which is being computed for `i`
// This works because each `dp[i][j]` value depends only on `dp[i-1][j]`, `dp[i][j-1]`, and `dp[i-1][j-1]`,
// all of which are accessible within these two rows.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let word1 = lines.next().expect("Error reading t").as_bytes();
    let word2 = lines.next().expect("Error reading t").as_bytes();

    let mut dp = vec![0; word2.len() + 1];

    // first row, meaning way to convert word1 of len 0 to word2 at different sizes.
    for i in 0..=word2.len() {
        dp[i] = i;
    }

    for i in 1..=word1.len() {
        let mut new_dp = vec![0; word2.len() + 1];
        new_dp[0] = i; // Converting `word1[0..i]` to an empty `word2` requires `i` deletions.
        for j in 1..=word2.len() {
            if word1[i - 1] == word2[j - 1] {
                new_dp[j] = dp[j - 1];
            } else {
                new_dp[j] = 1 + dp[j].min(new_dp[j - 1]).min(dp[j - 1]);
            }
        }
        dp = new_dp;
    }

    writeln!(out, "{}", dp[dp.len() - 1]).expect("error writing line");
    out.flush().ok();
}
