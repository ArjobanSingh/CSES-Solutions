use std::io::{self, BufWriter, Read, Write};

// Basially trick is maintain a Frequence array of nums which are maintaing consecutive sequence.
// And at every num, check if there's present a number 1 smaller than it available till now?
// If yes, meaning this number can be picked with the previous number.
// and further more we don't need to keep the previous number now, so remove it(make it false in freq array)
// and add this new number.(true this new number)
// Else if num - 1 is not present, meaning this will start a new beginning.
// of consecutive nums bigger than it. So put it in set.
// at last give the size of unique nums in set. That will be the number of turns we need.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let mut out = BufWriter::new(io::stdout().lock());

    lines.next().expect("Error reading n");

    let nums: Vec<usize> = lines
        .next()
        .expect("Error reading line")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let mut freq_arr: Vec<bool> = vec![false; nums.len() + 1];

    for num in nums {
        let prev = num - 1;
        if freq_arr[prev] {
            freq_arr[prev] = false;
        }
        freq_arr[num] = true;
    }

    writeln!(
        out,
        "{}",
        freq_arr
            .iter()
            .fold(0, |acc, &res| if res { acc + 1 } else { acc })
    )
    .expect("Error writing ans");

    out.flush().ok();
}
