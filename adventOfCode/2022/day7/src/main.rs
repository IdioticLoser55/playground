use std::{fs, collections::HashMap, time::Instant};

fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();

    let data = get_data("day7/input.txt");
    let data = parse_filesystem(&data);
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

fn part_1(dirs: &HashMap<String, i32>) -> i32{
    return dirs.values().filter(|v| **v <= 100_000).sum();
}

fn part_2(dirs: &HashMap<String, i32>) -> i32{
    let required_space = 30_000_000 - (70_000_000 - dirs.get("/").unwrap());
    return *dirs.values().filter(|v| **v >= required_space).min().unwrap();
}

fn parse_filesystem(data: &Vec<String>) -> HashMap<String, i32> {
    let mut current_dir_vec: Vec<String> = Vec::new();
    let mut current_dir_str: String = current_dir_vec.join("/");
    let mut size: i32;
    let mut dirs: HashMap<String, i32> = HashMap::new();
    
    for line in data {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();

        match line_split[0] {
            "$" => match line_split[1] {
                "cd" => match line_split[2] {
                    ".." => {
                        size = dirs.get(&current_dir_str).unwrap().clone();
                        current_dir_vec.pop();
                        current_dir_str = current_dir_vec.join("/");
                        *dirs.get_mut(&current_dir_str).unwrap() += size;
                    },
                    _ => {
                        current_dir_vec.push(line_split[2].to_string());
                        current_dir_str = current_dir_vec.join("/");
                        dirs.insert(current_dir_str.clone(), 0);
                    }, 
                },
                _ => (),
            },
            "dir" => (),
            _ => *dirs.get_mut(&current_dir_str).unwrap() += line_split[0].parse::<i32>().unwrap(),
        };

    }
    
    for _i in 1..current_dir_vec.len() {
        size = dirs.get(&current_dir_str).unwrap().clone();
        current_dir_vec.pop();
        current_dir_str = current_dir_vec.join("/");
        *dirs.get_mut(&current_dir_str).unwrap() += size;
    }
    
    return dirs;
}

fn get_data(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}