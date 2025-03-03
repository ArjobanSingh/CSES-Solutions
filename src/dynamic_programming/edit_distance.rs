use std::io::{self, BufWriter, Read, Write};

// TC: A*B, where A and B is the length of the A and B array
// SC: A*B, TO refer 1d Space solution, refer below
fn main_2d() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let word1 = lines.next().expect("Error reading t");
    let word2 = lines.next().expect("Error reading t");

    // dp[i][j] represents the minimum number of operations needed
    // to convert the first `i` characters of `word1` into the first `j` characters of `word2`.
    let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

    // Base Cases:
    // If one of the strings is empty, the only option is to insert/delete all characters of the other string.
    for j in 1..=word2.len() {
        // 1, 2, 3, 4
        dp[0][j] = j; // Inserting `j` characters in empty word1 to match `word2`
    }

    // first col, meaning way to convert word2 of len 0 to word1 at different sizes.
    for i in 1..=word1.len() {
        dp[i][0] = i; // Deleting `i` characters from word1 to match empty `word2`
    }

    for i in 1..dp.len() {
        for j in 1..dp[0].len() {
            if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                // If the characters are the same, no operation is needed.
                // We inherit the cost from dp[i-1][j-1] (previous subproblem).
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // Otherwise, choose the best transformation among three possible operations:
                // - `dp[i-1][j]`: Removing `word1[i-1]` to match `word2` (i.e., delete operation)
                // - `dp[i][j-1]`: Adding `word2[j-1]` to `word1` (i.e., insert operation)
                // - `dp[i-1][j-1]`: Replacing `word1[i-1]` with `word2[j-1]` (i.e., replace operation)
                //
                // We take the minimum of these three and add 1 because we are performing an operation.
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    writeln!(out, "{}", dp[dp.len() - 1][dp[0].len() - 1]).expect("error writing line");
    out.flush().ok();
}

// Space Optimization:
// Instead of using a 2D DP table, we only maintain two rows at a time:
// - The previous row (`dp`), which represents `dp[i-1][j]` values
// - The current row (`new_dp`), which is being computed for `i`
// This works because each `dp[i][j]` value depends only on `dp[i-1][j]`, `dp[i][j-1]`, and `dp[i-1][j-1]`,
// all of which are accessible within these two rows.
// SC: O(A + B)
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
