use std::io::{self, BufWriter, Read, Write};

// NOTE: This problem can be easilly solved using Ordered Sets or RBTrees which allows
// removing items at certain index in O(lgN) times and still maintains the sorting order.
// As no direct DS available in Rust and implementing RBTrees is more tricky than Segment trees.

// Step-by-step approach to solving the Josephus Problem II using a Segment Tree:
// 1. Build a Segment Tree to represent the children, where each node tracks the count of "alive" children in a range.
//    Initially, all children are marked as alive (value = 1).
// 2. Use modular arithmetic to calculate the index of the k-th child to be removed at each step.
//    This ensures efficient circular traversal around the remaining children.
// 3. Query the Segment Tree to find the position of the k-th alive child. This involves:
//    - Navigating the tree to identify the exact position in O(log n) time.
//    - Marking the child as "removed" by updating the tree.
// 4. Repeat the process until all children are removed, keeping track of the elimination order.
// 5. Output the result as the order in which children are eliminated.
// This approach leverages the Segment Tree for efficient range queries and updates,
// achieving an overall time complexity of O(n log n).

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n * 4 + 1],
        }
    }

    // track no. of nodes in each range
    fn build(&mut self, node: usize, l: usize, r: usize) {
        if l == r {
            self.tree[node] = 1;
            return;
        }

        let mid = l + (r - l) / 2;
        self.build(node * 2, l, mid);
        self.build(node * 2 + 1, mid + 1, r);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn query_and_update(&mut self, node: usize, l: usize, r: usize, index: usize) -> usize {
        if l == r {
            self.tree[node] = 0;
            return l;
        }

        let mid = l + (r - l) / 2;
        let l_child = node * 2;
        let r_child = node * 2 + 1;

        // we are using 0 based indexing for querying, so only go to left tree, if index is smaller than
        // no. of live children in left tree. Else for equal or more go to right tree.
        let ans = if self.tree[l_child] > index {
            self.query_and_update(l_child, l, mid, index)
        } else {
            self.query_and_update(r_child, mid + 1, r, index - self.tree[l_child])
        };

        self.tree[node] = self.tree[l_child] + self.tree[r_child];
        ans
    }

    fn live_count(&mut self) -> usize {
        self.tree[1] as usize
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
    let constraints: Vec<usize> = lines
        .next()
        .expect("Error reading n")
        .split_ascii_whitespace()
        .filter_map(|it| it.parse().ok())
        .collect();

    let n = constraints[0];
    let k = constraints[1];
    let mut idx = 0;

    let mut tree = SegmentTree::new(n);
    tree.build(1, 1, n);

    while tree.live_count() != 0 {
        idx += k;
        idx %= tree.live_count();
        let ans = tree.query_and_update(1, 1, n, idx);
        write!(out, "{ans} ").expect("error writing num");
    }

    writeln!(out, "").expect("error writing line");

    out.flush().ok();
}
