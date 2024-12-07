use std::fs;
use itertools::Itertools;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let bytes = input.as_bytes().trim_ascii();
    let parsed_lines = bytes.split(|c| *c == b'\n')
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
        });

    let mut correct_count = 0;
    for (result, equation_numbers) in parsed_lines {
        if test_equation(result, equation_numbers) {
            correct_count += result;
        }
    }

    println!("{}", correct_count);
}

fn parse_number(number: &[u8]) -> usize {
    number.iter()
        .fold(0, |acc, digit| {
            acc * 10 + usize::from(*digit) - usize::from(b'0')
        })
}

fn test_equation(result: usize, equation_numbers: Vec<usize>) -> bool {
    let operations = [1, 2, 3];
    let num_operations = equation_numbers.len() - 1;

    // let thing = operations.iter().combinations_with_replacement(num_operations).collect_vec();
    let possibilities = itertools::repeat_n(operations, num_operations).multi_cartesian_product();
    possibilities
        .into_iter()
        .any(|p| result == evaluate_equation(&equation_numbers, &p))
}

fn evaluate_equation(numbers: &[usize], operations: &[u8]) -> usize {
    let mut total = numbers[0];
    for (n, o) in numbers[1..].iter().zip(operations) {
        match o {
            1 => total *= n,
            2 => total += n,
            3 => total = total * 10_usize.pow(n.checked_ilog10().unwrap_or(0) + 1) + n,
            _ => panic!("unexpected operation"),
        };
    }

    total
}
