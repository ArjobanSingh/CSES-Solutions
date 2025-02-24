use std::{
    cmp,
    collections::HashMap,
    io::{self, BufWriter, Read, Write},
    usize,
};

// const MOD: i32 = 1_000_000_007;

fn backtrack(
    start_idx: usize,
    books: &Vec<usize>,
    max_money: usize,
    mut cur_result: usize,
    result: &mut usize,
    prices_map: &mut HashMap<usize, Collection>,
) {
    if max_money <= 0 || start_idx >= books.len() {
        return;
    }

    for idx in start_idx..books.len() {
        let book = books[idx];

        if let Some(collection) = prices_map.get(&book) {
            if collection.start >= collection.books.len() {
                continue;
            }

            // start idx is efficient way to get the next available book with lowest price
            // as all previous better prices for the book with these pages are used.
            let price = collection.books[collection.start];
            if price <= max_money {
                cur_result += book;
                *result = cmp::max(*result, cur_result);

                if let Some(collection) = prices_map.get_mut(&book) {
                    collection.start += 1;
                }

                // pick this once
                backtrack(
                    idx + 1,
                    books,
                    max_money - price,
                    cur_result,
                    result,
                    prices_map,
                );

                cur_result -= book;
                if let Some(collection) = prices_map.get_mut(&book) {
                    collection.start -= 1;
                }
            }
        }
    }
}

struct Collection {
    books: Vec<usize>,
    start: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();

    let mut out = BufWriter::new(io::stdout().lock());
    let input: Vec<usize> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // let n = input[0];
    let x = input[1];

    let prices: Vec<usize> = lines
        .next()
        .expect("Error reading prices")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    // pages to prices map desc
    let mut prices_map: HashMap<usize, Collection> = HashMap::with_capacity(x);
    let mut books: Vec<usize> = Vec::with_capacity(x);

    for (idx, book) in lines
        .next()
        .expect("Error getting pages")
        .split_ascii_whitespace()
        .enumerate()
    {
        let book: usize = book.parse().expect("Error parsing pages");
        let collection = prices_map.entry(book).or_insert(Collection {
            books: vec![],
            start: 0,
        });

        match collection.books.binary_search(&prices[idx]) {
            Ok(pos) | Err(pos) => collection.books.insert(pos, prices[idx]),
        }

        books.push(book);
    }
    books.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    backtrack(0, &books, x, 0, &mut result, &mut prices_map);

    writeln!(out, "{}", result).expect("error writing line");

    out.flush().ok();
}
