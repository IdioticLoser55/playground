use std::{fs, time::Instant};

fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();

    let data = get_data("day5/resources/input.txt");
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

fn part_1(data: &(Vec<Vec<char>>, Vec<Vec<usize>>)) -> String{
    let mut stacks = data.0.clone();
    let moves = &data.1;
    
    for action in moves {
        for _i in 0..action[0] {
            let tmp = stacks[action[1] - 1].pop().unwrap();
            stacks[action[2] - 1].push(tmp);
        }        
    }
    
    stacks.iter().map(|s| s[s.len() - 1]).collect::<String>()
}

fn part_2(data: &(Vec<Vec<char>>, Vec<Vec<usize>>)) -> String{
    let mut stacks = data.0.clone();
    let moves = &data.1;
    
    for action in moves {
        let stack = action[1] - 1;
        let stack_len = stacks[stack].len();
        let mut tmp = stacks[stack].split_off(stack_len - action[0]);
        stacks[action[2] - 1].append(&mut tmp);
    }
    
    stacks.iter().map(|s| s[s.len() - 1]).collect::<String>()
}

fn get_data(name: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let binding = fs::read_to_string(name).unwrap();
    let (stacks, moves) = binding.trim_end().split_once("\n\n").unwrap();
    let mut stacks = stacks.lines()
        .map(|s| {
            let mut chars = s.chars();
            chars.next();
            chars.step_by(4)
            .collect::<Vec<char>>()})
        .collect::<Vec<Vec<char>>>();
    stacks.pop();
    stacks = transpose(&stacks);

    let moves = moves.lines()
        .map(|s| s.trim().chars()
            .filter(|c| (c >= &'0' && c <= &'9') || c == &' ').collect::<String>()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    
    (stacks, moves)
}

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::with_capacity(v[0].len());
    
    for x in 0..result.capacity() {
        result.push(Vec::<char>::with_capacity(v.len()));
        for y in 0..v.len() {
            if v[v.len() - 1 - y][x]  == ' ' { break; }
            result[x].push(v[v.len() - 1 - y][x]);
        }
    }
    
    result
}