use std::{
    cmp,
    io::{self, BufWriter, Read, Write},
};

const MOD: i32 = 1_000_000_007;

fn backtrack(arr: &mut Vec<i32>, idx: usize, u_bound: i32, result: &mut i32) {
    if idx >= arr.len() {
        *result = (*result + 1) % MOD;
        return;
    }

    if arr[idx] != 0 {
        backtrack(arr, idx + 1, u_bound, result);
    } else if idx == 0 {
        // at the start
        let next = arr[idx + 1];
        if next == 0 {
            // all of the nums from 1 to u_bound can be used
            (1..=u_bound).for_each(|num| {
                arr[idx] = num;
                backtrack(arr, idx + 1, u_bound, result);
                arr[idx] = 0;
            });
        } else {
            ((next - 1)..=(next + 1))
                .filter(|&n| n > 0 && n <= u_bound)
                .for_each(|num| {
                    arr[idx] = num;
                    backtrack(arr, idx + 1, u_bound, result);
                    arr[idx] = 0;
                });
        }
    } else if idx == arr.len() - 1 || arr[idx + 1] == 0 {
        let prev = arr[idx - 1];
        // prev no will never be 0 as per our logic.
        ((prev - 1)..=(prev + 1))
            .filter(|&n| n > 0 && n <= u_bound)
            .for_each(|num| {
                arr[idx] = num;
                backtrack(arr, idx + 1, u_bound, result);
                arr[idx] = 0;
            });
    } else {
        // in middle and next element is not 0
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
            _ => return,
        };

        (start..=end)
            .filter(|&n| n > 0 && n <= u_bound)
            .for_each(|num| {
                arr[idx] = num;
                backtrack(arr, idx + 1, u_bound, result);
                arr[idx] = 0;
            });
    }
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

    // let n = input[0];
    let u_bound = input[1]; //  upper bound

    let mut items: Vec<i32> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let mut result = 0;
    if items.len() == 1 {
        writeln!(out, "{}", if items[0] == 0 { u_bound } else { 1 }).expect("error writing line");
        out.flush().ok();
        return;
    }

    backtrack(&mut items, 0, u_bound, &mut result);

    writeln!(out, "{result}").expect("error writing line");
    out.flush().ok();
}
