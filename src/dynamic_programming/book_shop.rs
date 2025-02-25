use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
    usize,
};

// This code solves the classic 0/1 Knapsack problem using a 2D dynamic programming approach.
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
    let prices: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let pages: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // 2D DP for knapsack.
    let mut dp = vec![vec![0; x + 1]; n];

    // current row(r) represent the r + 1books of the available books in consideration
    // avail_price is the col(or weight)
    for cur_item in 0..dp.len() {
        let price = prices[cur_item]; // weight
        let value = pages[cur_item];
        for avail_price in 1..dp[0].len() {
            // Check if we can even pick this value(based on weight)
            // and available max(which is c)
            if avail_price < (price as usize) {
                // we can't pick this put whatever max is possible from prev row
                if cur_item > 0 {
                    dp[cur_item][avail_price] = dp[cur_item - 1][avail_price];
                }
                continue;
            }

            // Check what max value we can get by picking it(and getting max from remaining weight);
            let picked_value = value
                + if cur_item > 0 {
                    dp[cur_item - 1][avail_price - price as usize]
                } else {
                    0
                };

            let skip_value = if cur_item > 0 {
                dp[cur_item - 1][avail_price]
            } else {
                0
            };
            dp[cur_item][avail_price] = cmp::max(picked_value, skip_value);
        }
    }

    writeln!(out, "{}", dp[n - 1][x]).expect("error writing line");
    out.flush().ok();
}
