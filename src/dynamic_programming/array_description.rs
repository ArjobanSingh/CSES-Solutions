use std::io::{self, BufWriter, Read, Write};

const MOD: i32 = 1_000_000_007;

/*
Approach:
- We use **Dynamic Programming (DP)** to count the number of valid ways to fill the array.
- Define `dp[i][x]` as the number of ways to fill the first `i+1` elements, where `nums[i] = x`.
- If `nums[i] == 0`, it can be any value from `1` to `u_bound`, otherwise, it must be `nums[i]`.
- Transition: `dp[i][x] = dp[i-1][x-1] + dp[i-1][x] + dp[i-1][x+1]` (if within range).
- Base case: If `nums[0] == 0`, initialize `dp[0][1..=u_bound] = 1`, else only `dp[0][nums[0]] = 1`.
- Final answer: Sum over `dp[n-1][x]` for all valid `x`.

  TC: (N * U)
  SP: (N * U), this can be improved to (2U), Look the 2nd solution below
*/
fn main_2D() {
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

    let n = input[0];
    let u_bound = input[1]; //  upper bound

    let nums: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // Base cases
    let mut dp: Vec<Vec<i32>> = vec![vec![0; u_bound + 1]; n];
    if nums[0] == 0 {
        // If nums[0] is unknown (0), it can be any value from 1 to u_bound
        dp[0].fill(1);
        dp[0][0] = 0; // 0 itself is not a valid number, just created dp of u_bound + 1 for easy traversal
    } else {
        // If nums[0] is fixed, set only dp[0][nums[0]] = 1
        dp[0].fill(0);
        dp[0][nums[0] as usize] = 1;
    }

    for i in 1..nums.len() {
        let x = nums[i] as usize;

        // Determine range of values nums[i] can take
        let st = if x == 0 { 1 } else { x };
        let end = if x == 0 { u_bound } else { x };

        (st..=end).for_each(|x| {
            // Sum possible ways from previous index (x-1, x, x+1)
            dp[i][x] = (x - 1..=x + 1).fold(0, |acc, x| {
                if x != 0 && x <= u_bound {
                    (acc + dp[i - 1][x]) % MOD
                } else {
                    acc
                }
            });
        });
    }

    writeln!(
        out,
        "{}",
        dp[n - 1].iter().fold(0, |acc, it| { (acc + it) % MOD })
    )
    .expect("error writing line");
    out.flush().ok();
}

// SC: O(U) - We only need the previous row's values (dp[i-1]) to compute the current row (dp[i]),
// so we maintain a single dp array and update it iteratively in each step.
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

    let u_bound = input[1]; //  upper bound

    let nums: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // Base cases
    let mut dp: Vec<i32> = vec![0; u_bound + 1];
    if nums[0] == 0 {
        // If nums[0] is unknown (0), it can be any value from 1 to u_bound
        dp.fill(1);
        dp[0] = 0; // 0 itself is not a valid number, just created dp of u_bound + 1 for easy traversal
    } else {
        // If nums[0] is fixed, set only dp[0][nums[0]] = 1
        dp.fill(0);
        dp[nums[0] as usize] = 1;
    }

    for i in 1..nums.len() {
        let mut new_dp = vec![0; u_bound + 1];
        let x = nums[i] as usize;

        // Determine range of values nums[i] can take
        let st = if x == 0 { 1 } else { x };
        let end = if x == 0 { u_bound } else { x };

        (st..=end).for_each(|x| {
            // Sum possible ways from previous index (x-1, x, x+1)
            new_dp[x] = (x - 1..=x + 1).fold(0, |acc, x| {
                if x != 0 && x <= u_bound {
                    (acc + dp[x]) % MOD
                } else {
                    acc
                }
            });
        });

        dp = new_dp;
    }

    writeln!(out, "{}", dp.iter().fold(0, |acc, it| { (acc + it) % MOD }))
        .expect("error writing line");
    out.flush().ok();
}
