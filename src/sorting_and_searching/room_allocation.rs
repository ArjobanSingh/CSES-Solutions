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

/// The approach starts by sorting the guests based on their arrival time, departure time, and index (to handle ties).
// A Min-Heap (Priority Queue) is used to track the guests in order of their departure time, then arrival time, and index.
// As we iterate through each guest, we check if there is any guest whose departure time is before the current guest's arrival.
// If such a guest exists, we assign that guest's room to the current guest. Otherwise, we allocate a new room (based on the current queue size + 1).
// At the end, the number of rooms needed is equal to the size of the priority queue, which holds the current state of the room allocations.
// Additionally, we maintain the room assignments in the result array.
// Note: The exact room assignment may vary, as any available room can be assigned to a guest when multiple rooms are free at the same time.
// The only requirement is to provide any valid room allocation order.
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
