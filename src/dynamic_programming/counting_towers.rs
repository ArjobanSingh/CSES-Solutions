use std::io::{self, BufWriter, Read, Write};

const MOD: u64 = 1_000_000_007;

/*
    Intuition:
    The problem involves building a tower layer by layer. At each layer, we can place:
    1. A single tile of width 2.
    2. Two tiles of width 1.

    Base Case:
    - For a single layer (n = 1), there is exactly:
      - 1 way to place a tile of width 2.
      - 1 way to place two tiles of width 1.

    Transition Logic:
    If the previous layer had two tiles of width 1, we can extend in 5 ways:
    1. Extend both previous tiles upwards.
    2. Extend the left tile and add a new single tile on the right.
    3. Extend the right tile and add a new single tile on the left.
    4. Place a new wide tile (width 2) on top of both.
    5. Place two new tiles of width 1 on top.

    If the previous layer had a single tile of width 2, we can extend in 3 ways:
    1. Extend it upwards.
    2. Block it and place a new tile of width 2.
    3. Block it and place two new tiles of width 1.

    Recurrence Relation:
    - dp[i][0]: Ways to build the tower ending with two tiles of width 1.
      - 4 ways to extend when the previous layer was two tiles of 1w
      - 1 way to extend when the previous layer was a single tile of 2w
      - Total: dp[i][0] = 4 * dp[i-1][0] + dp[i-1][1]

    - dp[i][1]: Ways to build the tower ending with a single tile of width 2.
      - 2 ways to extend when the previous layer was a single tile of 2w.
      - 1 way to extend when the previous layer was two tiles of 1w
      - Total: dp[i][1] = 2 * dp[i-1][1] + dp[i-1][0]

    Final Answer:
    Since we need to count all valid tower configurations of height n, the result is:
    dp[n-1][0] + dp[n-1][1] (mod MOD)
*/

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    lines.next().expect("Error reading t");

    let mut dp: Vec<[u64; 2]> = vec![[0; 2]; 1_000_000];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..dp.len() {
        let prev = dp[i - 1];
        dp[i][0] = (4 * prev[0] + prev[1]) % MOD;
        dp[i][1] = (2 * prev[1] + prev[0]) % MOD;
    }

    for line in lines {
        let n: usize = line.parse().expect("error parsing n");
        writeln!(out, "{}", (dp[n - 1][0] + dp[n - 1][1]) % MOD).expect("error writing line");
    }

    out.flush().ok();
}
