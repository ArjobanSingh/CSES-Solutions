use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
    usize,
};

// const MOD: i32 = 1_000_000_007;

// This code solves the classic 0/1 Knapsack problem using a 2D dynamic programming approach.
// For Space efficient approach of using 1D array, refer the 2nd solution below.
// The DP table `dp` has dimensions N × M, where:
// - `N` (rows) represents the number of books available.
// - `M` (columns) represents the total budget (`x`), which acts as the maximum weight/capacity constraint.
//
// The goal is to maximize the total number of pages while staying within the given budget.
//
// **Approach:**
// - For each book, we decide whether to pick it (if its price is within the remaining budget)
//   or skip it (keeping the budget unchanged).
// - If we pick the book, we add its pages to the best value achievable with the remaining budget.
// - If we skip the book, we retain the best value from the previous row.
// - The final answer is stored in `dp[n-1][x]`, representing the maximum pages obtainable with `n` books and budget `x`.
//
// **Time Complexity:** O(N * M), where `N` is the number of books and `M` is the budget limit.
// **Space Complexity:** O(N * M).
fn main_2d() {
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
    let x = input[1];

    // Weight for knapsack
    let weights: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let values: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // 2D DP for knapsack. Using 1 extra top row(for 0 items) to make code easier
    // where we don't have to manually check if r > 0
    let mut dp = vec![vec![0; x + 1]; n + 1];

    // current row(r) represent the r + 1books of the available books in consideration
    // avail_price is the col(or weight)
    for r in 1..dp.len() {
        let weight = weights[r - 1];
        let value = values[r - 1];
        for avail_weight in 1..dp[0].len() {
            if avail_weight < (weight as usize) {
                // we can't pick this put whatever max is possible from prev row
                dp[r][avail_weight] = dp[r - 1][avail_weight];
                continue;
            }

            // Check what max value we can get by picking it(and getting max from remaining weight);
            let picked_value = value + dp[r - 1][avail_weight - weight as usize];
            let skip_value = dp[r - 1][avail_weight];
            dp[r][avail_weight] = cmp::max(picked_value, skip_value);
        }
    }

    writeln!(out, "{}", dp[n][x]).expect("error writing line");
    out.flush().ok();
}

// The approach is same as above, only optimisation is using 1D array of only x + 1 items
// Instead of using (N + 1) * (X + 1) 2D array. Because if check carefully, we only need
// previous row's values to calculate current row. We don't need all of the other previous or next rows
// Just the strict previous row. So just maintain that.
// **Time Complexity:** O(N * M), where `N` is the number of books and `M` is the budget limit.
// **Space Complexity:** O(M).
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

    let n = input[0];
    let x = input[1];

    // Weight for knapsack
    let weights: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let values: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // 1D DP for knapsack.
    let mut dp = vec![0; x + 1];

    for r in 0..n {
        let weight = weights[r];
        let value = values[r];
        let mut new_dp = vec![0; x + 1];

        for avail_weight in 1..dp.len() {
            if avail_weight < (weight as usize) {
                // we can't pick this put whatever max is possible from prev row
                new_dp[avail_weight] = dp[avail_weight];
                continue;
            }

            // Check what max value we can get by picking it(and getting max from remaining weight);
            let picked_value = value + dp[avail_weight - weight as usize];
            let skip_value = dp[avail_weight];
            new_dp[avail_weight] = cmp::max(picked_value, skip_value);
        }
        dp = new_dp;
    }

    writeln!(out, "{}", dp[x]).expect("error writing line");
    out.flush().ok();
}
