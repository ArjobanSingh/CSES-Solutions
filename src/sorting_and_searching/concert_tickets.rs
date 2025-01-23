use std::{
    collections::BTreeMap,
    io::{self, BufWriter, Read, Write},
};

// The logic is to store the tickets in BTreeMap, which will keep them in sorted
// order based on there price, and we will keep there count as value.
// Next go over all the customer prices and get lower_bound if match else, 1 prev than lower bound. from BTree for that value
// and also decrement/remove the ticket after use.
// Retreival of upper_bound and removal is O(lgN) in BTree.
// So total TC: O(NlgN + MlgN)
// NlgN -> BTreeMap creation cost for N tickets
// M * lgM -> For M prices, make lgN operations on the Btree.
fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    lines.next().expect("Error reading n");

    let mut tickets: BTreeMap<u32, u32> = BTreeMap::new();

    for num in lines
        .next()
        .expect("Error reading tickets")
        .split_ascii_whitespace()
    {
        let num: u32 = num.parse().expect("error parsing ticket");
        tickets.entry(num).and_modify(|p| *p += 1).or_insert(1);
    }

    for price in lines
        .next()
        .expect("Error reading tickets")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse::<u32>().ok())
    {
        // get lower_bound if match else, 1 prev than lower bound.
        let elem = tickets.range(..=price).last().map(|(&k, &v)| (k, v));

        if let Some(el) = elem {
            writeln!(out, "{}", el.0).expect("error writing line");

            if let Some(value) = tickets.get_mut(&el.0) {
                *value -= 1; // Decrement the value
                if *value == 0 {
                    tickets.remove(&el.0); // Remove the key if the value is 0
                }
            }
        } else {
            writeln!(out, "-1").expect("error writing line");
        }
    }

    out.flush().ok();
}
