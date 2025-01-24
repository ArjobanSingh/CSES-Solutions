use std::{
    cmp,
    collections::BinaryHeap,
    io::{self, BufWriter, Read, Write},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    departure: u32,
    room: usize,
    idx: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // as we need to push this in Heap, which is by default MaxHeap, so we updating the ordering comparator
        // to return min value first. Kinda reverse() value to treat it as min heap
        other
            .departure
            .cmp(&self.departure)
            .then_with(|| other.room.cmp(&self.room))
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Approach:
/// 1. Sort the guests by arrival time, and in case of ties, by departure time and index.
/// 2. Use a min-heap (priority queue) to manage rooms based on:
///    - Departure time (priority for freeing up the room),
///    - Room number (to reuse the smallest available room),
///    - Guest index (to break ties consistently).
/// 3. For each guest:
///    - If the earliest-leaving guest's room is available (departure < arrival), reassign that room.
///    - Otherwise, allocate a new room based on the heap size.
/// 4. The total number of rooms needed is the size of the heap at the end.
/// 5. Maintain the room assignments in a result array and print it in the order of guest indices.
///
/// Note: The exact room assignments may vary since any valid allocation order is acceptable.
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
        .expect("error parsing n");

    let mut min_queue: BinaryHeap<State> = BinaryHeap::new();
    let mut stays: Vec<(u32, u32, usize)> = Vec::with_capacity(n);

    for (idx, line) in lines.enumerate() {
        let vec: Vec<u32> = line
            .split_ascii_whitespace()
            .filter_map(|it| it.parse().ok())
            .collect();

        let arrival = vec[0];
        let departure = vec[1];
        stays.push((arrival, departure, idx));
    }
    stays.sort();

    let mut result: Vec<usize> = vec![0; n];
    for (arrival, departure, idx) in stays {
        // if the person with earliest departure in queue is before
        // than this guest's arrival. Give that person's room to this
        // Else add this person in new room.
        if let Some(leaving_guest) = min_queue.peek().and_then(|it| {
            if it.departure < arrival {
                Some(it)
            } else {
                None
            }
        }) {
            result[leaving_guest.idx] = leaving_guest.room;
            min_queue.push(State {
                departure,
                room: leaving_guest.room,
                idx,
            });
            min_queue.pop();
        } else {
            min_queue.push(State {
                departure,
                room: min_queue.len() + 1,
                idx,
            });
        }
    }

    writeln!(out, "{}", min_queue.len()).expect("error writing line");
    while let Some(guest) = min_queue.pop() {
        result[guest.idx] = guest.room;
    }

    for room in result {
        write!(out, "{room} ").expect("error writing line");
    }
    writeln!(out, "").expect("error writing line");

    out.flush().ok();
}
