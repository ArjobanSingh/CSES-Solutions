use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

const MOD: i32 = 1_000_000_007;

fn backtrack(arr: &mut Vec<i32>, idx: usize, u_bound: i32, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if idx >= arr.len() {
        return 1;
    }

    if arr[idx] != 0 {
        if idx != 0 && arr[idx - 1].abs_diff(arr[idx]) > 1 {
            return 0;
        }
        return backtrack(arr, idx + 1, u_bound, dp);
    }

    if idx == 0 {
        // at the start
        let next = arr[idx + 1];
        if next == 0 {
            let mut cur_ans = 0;
            // all of the nums from 1 to u_bound can be used
            (1..=u_bound).for_each(|num| {
                arr[idx] = num;
                let value = backtrack(arr, idx + 1, u_bound, dp);
                cur_ans = (cur_ans + value) % MOD;
                arr[idx] = 0;
            });
            return cur_ans;
        }

        let mut cur_ans = 0;
        ((next - 1)..=(next + 1))
            .filter(|&n| n > 0 && n <= u_bound)
            .for_each(|num| {
                arr[idx] = num;
                let value = backtrack(arr, idx + 1, u_bound, dp);
                cur_ans = (cur_ans + value) % MOD;
                arr[idx] = 0;
            });
        return cur_ans;
    }

    if idx == (arr.len() - 1) || arr[idx + 1] == 0 {
        let mut cur_ans = 0;
        let prev = arr[idx - 1];
        // prev no will never be 0 as per our logic.
        ((prev - 1)..=(prev + 1))
            .filter(|&n| n > 0 && n <= u_bound)
            .for_each(|num| {
                if let Some(value) = dp[idx][num as usize] {
                    cur_ans = (cur_ans + value) % MOD;
                    return;
                }

                arr[idx] = num;
                let value = backtrack(arr, idx + 1, u_bound, dp);
                cur_ans = (cur_ans + value) % MOD;
                dp[idx][num as usize] = Some(value);
                arr[idx] = 0;
            });

        return cur_ans;
    }

    // in middle and next element is not 0
    let mut cur_ans = 0;
    let prev = arr[idx - 1];
    let next = arr[idx + 1];
    let diff = prev.abs_diff(next);
    let (start, end) = match diff {
        0 => (prev - 1, prev + 1),
        1 => {
            let smaller = cmp::min(prev, next);
            let larger = cmp::max(prev, next);
            (smaller, larger)
        }
        2 => {
            let smaller = cmp::min(prev, next);
            (smaller + 1, smaller + 1)
        }
        _ => return cur_ans,
    };

    (start..=end)
        .filter(|&n| n > 0 && n <= u_bound)
        .for_each(|num| {
            if let Some(value) = dp[idx][num as usize] {
                cur_ans = (cur_ans + value) % MOD;
                return;
            }

            arr[idx] = num;
            let value = backtrack(arr, idx + 1, u_bound, dp);
            cur_ans = (cur_ans + value) % MOD;
            dp[idx][num as usize] = Some(value);
            arr[idx] = 0;
        });
    return cur_ans;
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let input: Vec<i32> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let n = input[0];
    let u_bound = input[1]; //  upper bound

    let mut items: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    if items.len() == 1 {
        writeln!(out, "{}", if items[0] == 0 { u_bound } else { 1 }).expect("error writing line");
        out.flush().ok();
        return;
    }

    let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; u_bound as usize + 1]; n as usize];
    let result = backtrack(&mut items, 0, u_bound, &mut dp);

    writeln!(out, "{result}").expect("error writing line");
    out.flush().ok();
}
