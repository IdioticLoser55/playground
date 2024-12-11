use std::fs;
use std::time;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("reddit: {}\n", bench(&input, reddit));
    // println!("MyAttempt: {}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn reddit(input: &str) -> String {
    let part1 = solve(&input, 25);
    let part2 = solve(&input, 75);
    let test = solve(&input, 100);

    format!("Part 1: {part1}\nPart 2: {part2}\nTest: {test}")
}

fn solve(input: &str, steps: usize) -> usize {
    // because we're only counting the number of stones
    // and the rules don't rely on order we can discard any order.
    // we count each type of stone up into the hash map.
    // then when we bring out of the hashmap and loop through.
    // for each stone we can keep the count from before.
    // If we split we now have two numbers with that count.
    // and if we transform we now have a different nummber,
    // but still with the same count.
    // If that number happens to appear again for some reason,
    // The counts will then be joined in the hashmap.
    // and for the next iteration we now only have to perform one operation.
    // Instead of performing the operatioon for each instance of the number.

    
    // Sticks into a hashmap (stone number, number of stones).
    let mut stones: HashMap<u64, usize> = input
        .trim()
        .split_ascii_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    for _ in 0..steps {
        // extracts from hashmap and loops through all..
        for (stone, n) in stones.drain().collect::<Vec<_>>() {
            // function: for stone s. If already present increment count by n.
            // else insert n.
            let mut insert = |s| {
                stones.entry(s).and_modify(|x| *x += n).or_insert(n);
            };
            // swap 0 to 1.
            if stone == 0 {
                insert(1);
            } else {
                // performs match on number of digits.
                match (stone as f32).log10().floor() as u32 + 1 {
                    // if digits even split and insert.
                    digits if digits % 2 == 0 => {
                        insert(stone / 10u64.pow(digits / 2));
                        insert(stone % 10u64.pow(digits / 2));
                    }
                    // multiply.
                    _ => insert(stone * 2024),
                }
            }
        }
    }
    stones.values().sum()
}

fn my_attempt(input: &str) -> String {
    let num_iter = input
        .trim()
        .as_bytes()
        .split(|&c| c == b' ')
        .map(parse_number);

    let blinks_to_apply = 6;
    let mut stones_count = 0;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    for num in num_iter {
        queue.push((num, blinks_to_apply));

        while let Some((mut num, mut blinks_left)) = queue.pop() {
            loop {
                if blinks_left == 0 {
                    break;
                }
                blinks_left -= 1;

                if num == 0 {
                    num = 1;
                    continue;
                }

                let digits = num.ilog10() + 1;
                if digits % 2 == 0 {
                    let modifier = 10_usize.pow(digits / 2);
                    queue.push((num % modifier, blinks_left));
                    num = num / modifier;
                    continue;
                }

                num = num * 2024;
            }

            println!("{num}");
            stones_count += 1;
        }
    }

    println!("Stones Count: {stones_count}");


    "".to_string()
}

fn parse_number(number: &[u8]) -> usize {
    number.iter()
        .fold(0, |acc, digit| {
            acc * 10 + parse_digit(*digit)
        })
}

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
