use std::fs;
use std::time;
use regex::Regex;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("Regex:\n{}\n", bench(&input, regex));
    println!("Custom:\n{}\n", bench(&input, custom_parser));
    println!("Split:\n{}\n", bench(&input, split));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}


fn split(input: &str) -> String {
    fn handle_mul(input: &str) -> usize {
        input
            .split("mul(")
            .filter_map(|input| {
                let (input, _) = input.split_once(")")?;
                let (left, right) = input.split_once(",")?;
                let left = left.parse::<usize>().ok()?;
                let right = right.parse::<usize>().ok()?;
    
                if left > 1000 || right > 1000 {
                    return None;
                }
                Some(left * right)
            })
            .sum()
    }

    fn solve(input: &str) -> usize {
        let mut total = 0;
        let mut window = input;

        while let Some(dont_idx) = window.find("don't()") {
            let (valid, rest) = window.split_at(dont_idx);
            total += handle_mul(valid);

            let Some(do_idx) = rest.find("do()") else {
                return total;
            };
            window = &rest[do_idx..];
        }

        total + handle_mul(window)
    }

    let p1 = handle_mul(&input);
    let p2 = solve(&input);

    format!("Part1: {p1}\nPart2: {p2}")
}

fn custom_parser(input: &str) -> String {
    let memory = input.as_bytes();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    while index < memory.len() {
        if memory[index] != b'm' && memory[index] != b'd' {
            index += 1;
            continue;
        }


        if memory[index..].starts_with(b"mul(") {
            index += 4;
        } else if memory[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue
        } else {
            index += 1;
            continue
        }

        let mut first = 0;

        while memory[index].is_ascii_digit() {
            first = 10 * first + (memory[index] - b'0') as u32;
            index += 1;
        }


        if memory[index] != b',' {
            continue;
        }
        index += 1;

        let mut second = 0;

        while memory[index].is_ascii_digit() {
            second = 10 * second + (memory[index] - b'0') as u32;
            index += 1;
        }

        if memory[index] != b')' {
            continue;
        }
        index += 1;

        let product = first * second;
        part_one += product;
        if enabled {
            part_two += product;
        }
    }

    format!("Part 1: {part_one}\nPart 2: {part_two}")

}

fn regex(input: &str) -> String {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

    let (_, part1, part2) = re
        .captures_iter(&input)
        .fold((true, 0, 0), |(state, part1, part2), captures| match &captures[0] {
            "do()" => (true, part1, part2),
            "don't()" => (false, part1, part2),
            _ => {
                let n1 = &captures[2].parse::<usize>().unwrap();
                let n2 = &captures[3].parse::<usize>().unwrap();
                let product = n1 * n2;

                if !state {
                    (state, part1 + product, part2)
                } else {
                    (state, part1 + product, part2 + product)
                }
            }
        });

    format!("Part 1: {part1}\nPart2: {part2}")
}
