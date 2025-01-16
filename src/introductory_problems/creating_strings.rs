use std::io::{self, BufWriter, Read, Write};

// consumes more memory as we creating pending array always
fn backtrack_copy_pending(
    pending: Vec<char>,
    result: &mut Vec<Vec<char>>,
    cur: &mut Vec<char>,
    ans_size: usize,
) {
    if cur.len() == ans_size {
        result.push(cur.clone());
        return;
    }

    for (idx, &char) in pending.iter().enumerate() {
        if idx != 0 && char == pending[idx - 1] {
            // if same as prev, skip it
            continue;
        }

        let new_pending: Vec<char> = pending
            .iter()
            .enumerate()
            .filter_map(|(i, &ch)| if i != idx { Some(ch) } else { None })
            .collect();

        cur.push(char);
        backtrack_copy_pending(new_pending, result, cur, ans_size);
        cur.pop();
    }
}

// this would give us the permutations without creating new pending vector every time
// but they won't be in sorted order. So can't be used this to submit for solution.
// Though nice trick to use start idx and swap logic to find permutations.
fn backtrack_non_sorted_permutations(
    start: usize,
    pending: &mut Vec<char>,
    result: &mut Vec<Vec<char>>,
    cur: &mut Vec<char>,
    ans_size: usize,
) {
    if cur.len() == ans_size {
        result.push(cur.clone());
        return;
    }

    for idx in start..pending.len() {
        if idx > start && pending[idx] == pending[start] {
            // if same as start, skip it
            continue;
        }

        // swap this idx with start. so that all the pending chars go after swap
        pending.swap(start, idx);
        cur.push(pending[start]);

        backtrack(start + 1, pending, result, cur, ans_size);
        cur.pop();
        pending.swap(start, idx);
    }
}

fn backtrack(
    pending: &Vec<char>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<char>>,
    cur: &mut Vec<char>,
    ans_size: usize,
) {
    if cur.len() == ans_size {
        result.push(cur.clone());
        return;
    }

    for (idx, &char) in pending.iter().enumerate() {
        if used[idx] {
            continue;
        }

        if idx != 0 && char == pending[idx - 1] && !used[idx - 1] {
            // Skip this character if it's the same as the previous one (a duplicate)
            // AND the previous duplicate has not been used in the current recursive cycle(in parent's for loop).
            // This ensures that we process duplicates in order and avoid generating
            // duplicate permutations.
            //
            // Example: For "aabc", if we are at the second 'a' (idx = 1) in a nested
            // recursive call, and the first 'a' (idx = 0) has already been used in
            // a recursive cycle, we allow this 'a'. However, if the first 'a'
            // hasn't been used yet in this recursive cycle, we skip the second 'a' to maintain order.
            //
            // The key idea is to only skip if:
            // - This is a duplicate, AND
            // - The previous duplicate has not already been used in the current recursive cycle.
            // The `used` array ensures that characters used in current recursive cycle are tracked,
            // and it will be reset (set to false) properly as recursion backtracks.
            continue;
        }

        used[idx] = true;
        cur.push(char);
        backtrack(pending, used, result, cur, ans_size);
        cur.pop();
        used[idx] = false;
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

    let orignal = lines.next().expect("Error reading n").to_string();
    let mut chars: Vec<char> = orignal.chars().collect();
    chars.sort();

    let mut result: Vec<Vec<char>> = Vec::new();
    let mut cur: Vec<char> = Vec::with_capacity(chars.len());
    backtrack(
        &chars,
        &mut vec![false; orignal.len()],
        &mut result,
        &mut cur,
        orignal.len(),
    );

    writeln!(out, "{}", result.len()).expect("error writing size");
    for arr in result {
        for ch in arr {
            write!(out, "{ch}").expect("Error writing char");
        }
        writeln!(out).expect("error writing line");
    }

    out.flush().ok();
}
