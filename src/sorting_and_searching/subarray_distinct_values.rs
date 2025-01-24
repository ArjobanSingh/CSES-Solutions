use std::{
    collections::HashMap,
    io::{self, BufWriter, Read, Write},
};

// Steps to solve using the sliding window approach:
// 1. Use a sliding window with two pointers (`l` for left and `r` for right).
// 2. Expand the window by moving the right pointer (`r`) and include the current element in the window.
//    - Track the frequency of elements in the window using a map.
//    - If the count of unique elements exceeds `k`, shrink the window from the left by moving the left pointer (`l`)
//      and decrement the frequency of the element at `l`.
// 3. For each valid window (where unique elements ≤ `k`):
//    - Add the size of the window (`r - l + 1`) to the result.
//      This size represents the number of subarrays ending at the current position `r`.
// 4. Continue until the right pointer (`r`) has traversed the entire array.
// 5. The final result is the total count of valid subarrays.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let constraints: Vec<usize> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let n = constraints[0];
    let k = constraints[1];

    let arr: Vec<u32> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let mut map: HashMap<u32, usize> = HashMap::with_capacity(n);
    let mut l = 0;
    let mut ans = 0;

    for r in 0..n {
        map.entry(arr[r]).and_modify(|c| *c += 1).or_insert(1);

        while map.len() > k && l < r {
            if let Some(value) = map.get_mut(&arr[l]) {
                *value -= 1;
                if *value == 0 {
                    map.remove(&arr[l]); // Remove the key if the value is 0
                }
            }
            l += 1;
        }
        ans += r - l + 1;
    }

    writeln!(out, "{ans}").expect("error writing line");

    out.flush().ok();
}
