use std::fs;
use std::time;

fn main() {
    println!("Part1: {}", bench(part1));
    println!("Part2: {}\n", bench(part2));
}

fn bench(f: fn() -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn safe1(nums: &[i32]) -> bool {
    let ascending = nums[0] < nums[1];

    let mut prev = nums[0];
    for &n in &nums[1..] {
        let diff = (prev - n).abs();

        let no_diff = diff < 1;
        let too_big_gap = diff > 3;
        let wrong_direction = (ascending && prev > n) || (!ascending && prev < n);

        if no_diff || too_big_gap || wrong_direction {
            return false;
        }

        prev = n;
    }
    
    return true;
}

fn safe2(nums: &[i32]) -> bool {
    for skip_idx in 0..nums.len() {
        let first_half = &nums[..skip_idx];
        let second_half = &nums[skip_idx+1..];

        let skipped_nums: Vec<i32> = first_half
            .iter()
            .chain(second_half.iter())
            .map(|&n| n)
            .collect();
        if safe1(&skipped_nums) {
            return true
        }
    }

    false
}

fn part1() -> String {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    contents
        .lines()
        .into_iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            safe1(&nums)
        })
        .filter(|&result| result)
        .count()
        .to_string()
}

fn part2() -> String {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    contents
        .lines()
        .into_iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            safe2(&nums)
        })
        .filter(|&result| result)
        .count()
        .to_string()
}
