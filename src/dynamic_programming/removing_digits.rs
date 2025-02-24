use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

// This is DP approach and guarantees solution. On the other hand Greedy also
// can be used to solve this(which is shown below). But proving solution with
// that is not straight forward.
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

    let mut dp = vec![usize::MAX; n + 1];
    for i in 0..=(cmp::min(9, n)) {
        dp[i] = 1;
    }

    for i in 10..dp.len() {
        // For the current number i, check each of its digits.
        // Check the steps needed to reach (i - digit) from DP.
        // We need 1 more step than the steps needed to reach (i - digit).
        // For each digit, find the (i - digit) with the smallest steps.
        // Use that value and add 1 to it to get the steps for the current number.
        // Base case is sum 1-9 which takes only 1 step.
        let mut num = i;
        while num != 0 {
            let digit = num % 10;
            dp[i] = cmp::min(
                dp[i],
                if dp[i - digit] == usize::MAX {
                    usize::MAX
                } else {
                    dp[i - digit] + 1
                },
            );
            num /= 10;
        }
    }

    writeln!(out, "{}", dp[n]).expect("error writing line");

    out.flush().ok();
}

fn largest_digit(mut num: i32) -> i32 {
    let mut digit = 0;
    while num != 0 {
        digit = cmp::max(digit, num % 10);
        num = num / 10
    }
    digit
}

// GREEDY approach. Faster, but harder to provide the proof.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let mut n: i32 = lines
        .next()
        .expect("Error reading n")
        .parse()
        .expect("error parsing n");

    let mut result = 0;
    while n != 0 {
        let largest = largest_digit(n);
        n -= largest;
        result += 1;
    }

    writeln!(out, "{}", result).expect("error writing line");

    out.flush().ok();
}
