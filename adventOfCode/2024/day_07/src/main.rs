use std::time;
use std::fs;
use itertools::Itertools;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Operations {
    Add,
    Mul,
    Concat,
}

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("ME: {}", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn my_attempt(input: &str) -> String {
    let parsed_lines = parse_input(&input);

    let mut combinations: HashMap<usize, (Vec<Vec<Operations>>, Vec<Vec<Operations>>)> = HashMap::new();

    let mut part1_count = 0;
    let mut part2_count = 0;
    for (result, equation_numbers) in parsed_lines {
        let num_required_ops = equation_numbers.len() - 1;
        let (part1_combs, part2_combs) = combinations
            .entry(num_required_ops)
            .or_insert_with_key(|&k| generate_combinations(k));

        if part1_combs.iter().any(|comb| result == evaluate_equation(&equation_numbers, comb)) {
            part1_count += result;
        } else if part2_combs.iter().any(|comb| result == evaluate_equation(&equation_numbers, comb)) {
            part2_count += result;
        }
    }

    part2_count += part1_count;

    format!("Part 1: {part1_count}\nPart 2: {part2_count}")
}

fn generate_combinations(k: usize) -> (Vec<Vec<Operations>>, Vec<Vec<Operations>>) {
    let part1_operations = vec![Operations::Add, Operations::Mul];
    let part2_operations = vec![Operations::Add, Operations::Mul, Operations::Concat];

    let part1_combinations = itertools::repeat_n(part1_operations, k).multi_cartesian_product().collect_vec();
    let part2_combinations = itertools::repeat_n(part2_operations, k).multi_cartesian_product()
        .filter(|comb| !part1_combinations.contains(comb))
        .collect_vec();

    (part1_combinations, part2_combinations)
}

fn evaluate_equation(numbers: &[usize], operations: &[Operations]) -> usize {
    let mut total = numbers[0];
    for (n, o) in numbers[1..].iter().zip(operations) {
        match o {
            Operations::Mul => total *= n,
            Operations::Add => total += n,
            Operations::Concat => total = total * 10_usize.pow(n.checked_ilog10().unwrap_or(0) + 1) + n,
        };
    }

    total
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, Vec<usize>)> {
    let bytes = input.as_bytes().trim_ascii();
    bytes.split(|c| *c == b'\n')
        .map(|line| {
            let colon_pos = line.iter().position(|c| *c == b':').unwrap();
            let (result, equation_numbers) = line.split_at(colon_pos);

            let result = parse_number(result);
            let equation_numbers = equation_numbers[1..]
                .trim_ascii()
                .split(|c| *c == b' ')
                .map(|num| parse_number(num))
                .collect::<Vec<usize>>();

            (result, equation_numbers)
        })
}


fn parse_number(number: &[u8]) -> usize {
    number.iter()
        .fold(0, |acc, digit| {
            acc * 10 + usize::from(*digit) - usize::from(b'0')
        })
}
