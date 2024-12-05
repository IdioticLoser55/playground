use hashbrown::{HashMap, HashSet};
use std::fs;
use std::time;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("My Solution: {}\n", bench(&input, my_solution));
    println!("Reddit Sort: {}\n", bench(&input, reddit_sort_solution));
    println!(
        "Reddit Sort Cust: {}\n",
        bench(&input, reddit_sort_solution_customised)
    );
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn reddit_sort_solution(input: &str) -> String {
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    for l in s1.lines() {
        let (x, y) = l.split_once('|').unwrap();
        orderings
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }
    let pages = s2.lines().map(|l| {
        l.split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let (mut p1, mut p2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| orderings.get(b).map_or(false, |set| set.contains(a))) {
            p1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| orderings.get(b).map_or(false, |set| set.contains(a)).cmp(&true));
            p2 += p[p.len() / 2];
        }
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn reddit_sort_solution_customised(input: &str) -> String {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut rules_mapping: HashMap<u8, HashSet<u8>> = HashMap::new();

    rules
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mid = line.iter().position(|&b| b == b'|').unwrap();
            let (a, b) = line.split_at(mid);
            (
                a.iter().fold(0, |acc, &c| acc * 10 + c - b'0'),
                b[1..].iter().fold(0, |acc, &c| acc * 10 + c - b'0'),
            )
        })
        .for_each(|(a, b)| {
            rules_mapping
                .entry(a)
                .and_modify(|set| {
                    set.insert(b);
                })
                .or_insert(HashSet::new())
                .insert(b);
        });

    let pages = pages.as_bytes().split(|&b| b == b'\n').map(|line| {
        line.split(|&c| c == b',')
            .map(|num| num.iter().fold(0, |acc, &c| acc * 10 + c - b'0'))
            .collect::<Vec<_>>()
    });

    let (mut p1, mut p2): (usize, usize) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| rules_mapping.get(a).map_or(false, |set| set.contains(b))) {
            p1 += usize::from(p[p.len() / 2]);
        } else {
            p.sort_by(|a, b| {
                rules_mapping
                    .get(a)
                    .map_or(false, |set| set.contains(b))
                    .cmp(&true)
            });
            p2 += usize::from(p[p.len() / 2]);
        }
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn my_solution(input: &str) -> String {
    /*
     * Extract the rules from the page input.
     *
     * For some reason can't split bytes into lines.
     * And not entirely sure how to handle \r\n or \n, etc.
     * So splitting into lines then converting to bytes.
     *
     * Keep mapping until the line is empty.
     * return a tuple of two numbers (a, b) where a must come before b.
     * */
    let rules = input.lines().map(|line| line.as_bytes()).map_while(|line| {
        if line.is_empty() {
            None
        } else {
            Some((
                (line[0] - b'0') * 10 + line[1] - b'0',
                (line[3] - b'0') * 10 + line[4] - b'0',
            ))
        }
    });

    // put the rules into a hash map.
    // Hash map for each page with a hash set of the pages that come after.
    let mut rules_count = 0;
    let mut rules_map: HashMap<u8, HashSet<u8>> = HashMap::new();
    rules.for_each(|(before, after)| {
        rules_count += 1;
        rules_map
            .entry(before)
            .and_modify(|set| {
                set.insert(after);
            })
            .or_insert(HashSet::new())
            .insert(after);
    });

    // parse the updates.
    let updates = input
        .lines()
        // skip past rules.
        .skip(rules_count + 1)
        .map(|update| {
            update
                .as_bytes()
                .split(|&c| c == b',')
                .map(|num_slice| num_slice.iter().fold(0, |acc, b| acc * 10 + b - b'0'))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    // println!("\nUpdates:\n{:#?}", updates[1]);

    let mut valid_sum: usize = 0;
    let mut invalid_sum: usize = 0;
    updates.iter().for_each(|update| {
        let mut update_valid = true;
        let mut update_iter = update.iter();
        let mut correct_update: Vec<u8> = Vec::new();

        // get first elem.
        let mut prev = update_iter.next().unwrap();
        for (i, curr) in update_iter.enumerate() {
            // check if current elem comes after prev elem.
            if rules_map.get(prev).map_or(false, |set| set.contains(curr)) {
                prev = curr;
                continue;
            } else {
                // update is false.
                update_valid = false;

                // pull all the correct records in.
                correct_update.extend_from_slice(&update[correct_update.len()..i + 1]);

                // find where the current element is supposed to go.
                let idx = correct_update
                    .iter()
                    .rev()
                    .position(|page| rules_map.get(page).map_or(false, |set| set.contains(curr)))
                    .map_or(
                        0, // if not found then it must be first.
                        |pos| correct_update.len() - pos,
                    );

                // insert elem in correct position.
                correct_update.insert(idx, *curr);
            }
        }

        if update_valid {
            valid_sum += usize::from(update[update.len() / 2]);
        } else {
            //
            correct_update.extend_from_slice(&update[correct_update.len()..update.len()]);

            invalid_sum += usize::from(correct_update[correct_update.len() / 2]);
        }
    });

    format!("Part 1: {valid_sum}\nPart 2: {invalid_sum}")
}
