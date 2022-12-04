use std::{fs, time::Instant};

fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();

    let data = get_data("day4/resources/input.txt");
    let reading_time = timer.elapsed();
    
    timer = Instant::now();
    let part_1_ans = part_1(&data);
    let part_1_time = timer.elapsed();
    
    timer = Instant::now();
    let part_2_ans = part_2(&data);
    let part_2_time = timer.elapsed();
    let overall_time = overall_timer.elapsed();

    println!("The answer for part 1 is: {}", part_1_ans);
    println!("The answer for part 2 is: {}", part_2_ans);
    println!("The overall time is {:.2?}", overall_time);
    println!("The reading phase took {:.2?}, part 1 took {:.2?} and part 2 took {:.2?}", reading_time, part_1_time, part_2_time);
}

fn part_1(data: &Vec<String>) -> i32{
    data.iter().map(|line| {
        let (l, r) = line.split_once(",").unwrap();
        let l = parse_range(l);
        let r = parse_range(r);
        let o = l & r;
        
        ((o == l) || (o == r)) as i32
    }).sum()
}

fn part_2(data: &Vec<String>) -> i32{
    let mut count = 0;

    for line in data {
        let (l, r) = line.split_once(",").unwrap();
        let l = parse_range(l);
        let r = parse_range(r);
        
        if l & r > 0 {
            count += 1;
        }
    }
    
    count
}

fn parse_range(range: &str) -> u128 {
    let (l, r) = range.split_once("-").unwrap();
    let (l, r) = (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap());
    (1u128 << (r + 1)) - (1u128 << l)
}

fn get_data(name: &str) -> Vec<String> {
    fs::read_to_string(name).unwrap().trim().lines().map(|s| s.to_string())
        .collect::<Vec<_>>()
}