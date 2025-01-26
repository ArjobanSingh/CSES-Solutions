use std::io::{self, BufWriter, Read, Write};
// TODO: Need to improve solution

const MOD: i64 = 1_000_000_007;

fn recurse(target: i64, num: i64, dp: &mut Vec<Vec<i64>>) -> i64 {
    if target == 0 {
        return 1;
    }

    if num <= 0 || target < 0 {
        return 0;
    }

    if dp[num as usize][target as usize] >= 0 {
        return dp[num as usize][target as usize];
    }

    // include this num for sum
    let include = recurse(target - num, num - 1, dp);
    let exclude = recurse(target, num - 1, dp);

    let ans = (exclude + include) % MOD;
    dp[num as usize][target as usize] = ans;
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let n: i64 = lines
        .next()
        .expect("Error reading n")
        .parse()
        .expect("Error parsing n");

    let total_sum = n * (n + 1) / 2;
    if total_sum % 2 != 0 {
        writeln!(out, "0").expect("error writing line");
        out.flush().ok();
        return;
    }
    let target = total_sum / 2;

    let mut dp: Vec<Vec<i64>> = vec![vec![-1; target as usize + 1]; n as usize + 1];
    dp[0][0] = 1;

    let result = recurse(target, n, &mut dp);

    // Divide the result by 2 using modular arithmetic
    let result = (result * mod_inverse(2, MOD)) % MOD;

    writeln!(out, "{result}").expect("error writing line");

    out.flush().ok();
}

// Compute modular inverse using Fermat's little theorem
fn mod_inverse(a: i64, m: i64) -> i64 {
    mod_exp(a, m - 2, m)
}

// Compute modular exponentiation
fn mod_exp(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}
