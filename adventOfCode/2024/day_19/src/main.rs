use std::fs;
use std::time;
use std::collections::BTreeMap;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("My Attempt:\n{}\n", bench(&input, my_attempt));
    println!("Reddit Trie:\n{}\n", bench(&input, reddit_trie));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day19.rs
pub fn reddit_trie(input: &str) -> String {
    let (prefix, suffix) = input.trim().split_once("\n\n").unwrap();

    // Build Trie from all towels.
    let mut trie = Vec::with_capacity(1_000);
    trie.push(Node::new());

    for towel in prefix.split(", ") {
        let mut i = 0;

        for j in towel.bytes().map(perfect_hash) {
            if trie[i].next[j] == 0 {
                // This is a new prefix, so update the index to point to it then push new node.
                trie[i].next[j] = trie.len();
                i = trie.len();
                trie.push(Node::new());
            } else {
                // Follow existing prefix.
                i = trie[i].next[j];
            }
        }

        trie[i].towel = true;
    }

    let mut part_one = 0;
    let mut part_two = 0;
    let mut ways: Vec<usize> = Vec::with_capacity(100);

    for design in suffix.lines().map(str::as_bytes) {
        let size = design.len();

        // Reset state.
        ways.clear();
        ways.resize(size + 1, 0);

        // There's 1 way to create any possible first prefix.
        ways[0] = 1;

        for start in 0..size {
            // Only consider suffixes that have a valid prefix.
            if ways[start] > 0 {
                // Walk trie from root to leaf.
                let mut i = 0;

                for end in start..size {
                    // Get next link.
                    i = trie[i].next[perfect_hash(design[end])];

                    // This is not a valid prefix, stop the search.
                    if i == 0 {
                        break;
                    }

                    // Add the number of possible ways this prefix can be reached.
                    if trie[i].towel {
                        ways[end + 1] += ways[start];
                    }
                }
            }
        }

        // Last element is the total possible combinations.
        let total = ways[size];
        part_one += (total > 0) as u64;
        part_two += total;
    }

    format!("Part 1: {part_one}\nPart 2: {part_two}")
}

/// Hashes the five possible color values white (w), blue (u), black (b), red (r), or green (g)
/// to 6, 4, 0, 1 and 5 respectively. This compresses the range to fit into an array of 7 elements.
fn perfect_hash(b: u8) -> usize {
    (b as usize + (b as usize >> 4)) % 8
}

/// Simple Node object that uses indices to link to other nodes.
struct Node {
    towel: bool,
    next: [usize; 7],
}

impl Node {
    fn new() -> Self {
        Node { towel: false, next: [0; 7] }
    }
}

fn my_attempt(input: &str) -> String {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();

    let patterns = patterns
        .split(", ")
        .collect::<Vec<_>>();

    let mut indices: BTreeMap<usize, usize> = BTreeMap::new();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut match_count;

    // for each design.
    for line in designs.lines() {

        // reset and prime working vars.
        indices.clear();
        indices.insert(0, 1); // we have 1 match for nothing.
        match_count = 0;


        // loop on lowest idx until nothing left.
        while let Some((idx, count)) = indices.pop_first() {
            // check if complet match.
            if idx == line.len() {
                match_count = count;
                break;
            }

            // get remainder of design.
            let remaining = &line[idx..];

            // against each pattern
            for pattern in &patterns {
                // check for match.
                if remaining.starts_with(pattern) {
                    // add match and position.
                    let idx = idx + pattern.len();

                    indices.entry(idx)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }

        }

        if match_count > 0 {
            p1 += 1;
            p2 += match_count;
        }
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}
