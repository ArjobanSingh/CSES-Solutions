use std::io::{self, BufWriter, Read, Write};

// The idea is sort movies by there end time. By which we would have assurity that
// every i + 1 movie is end after ith movie. So start from 1 movie and check
// if it starts after or equal to the end time of last movie choose to watch and increment the counter.
// Initially the last movie we choose to watch would be 1st movie, as it's end time would be earliest.
// And keep the initial result 1, to cover the last valid movie or let's say first movie case.
// And also while iterating keep track of the end time of the last watched movie, as
// we need to compare only with that.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    lines.next().expect("Error reading n");
    let mut out = BufWriter::new(io::stdout().lock());

    let mut movies: Vec<(i32, i32)> = lines
        .map(|line| {
            let items: Vec<i32> = line
                .split_ascii_whitespace()
                .filter_map(|it| it.parse().ok())
                .collect();
            return (items[0], items[1]);
        })
        .collect();

    movies.sort_by(|a, b| a.1.cmp(&b.1));

    let mut result = 1;
    let mut cur_end = movies[0].1;
    for idx in 1..movies.len() {
        if movies[idx].0 >= cur_end {
            result += 1;
            cur_end = movies[idx].1;
        }
    }

    writeln!(out, "{result}").expect("error writing line");

    out.flush().ok();
}
