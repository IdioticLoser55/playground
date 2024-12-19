use std::fs;
use std::time;
use std::collections::BTreeMap;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("My Attempt:\n{}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
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
