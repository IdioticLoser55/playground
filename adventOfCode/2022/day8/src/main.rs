use std::{fs, time::Instant};

fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();

    let data = get_data("day8/input.txt");
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

fn part_1(data: &Vec<i32>) -> i32 {
    let mut chosen: Vec<i32> = vec![0; data.len()];
    let size = if data.len() > 25 {99} else {5};
    let mut tmp: i32;
    let mut left_to_right: i32;
    let mut right_to_left: i32;
    let mut top_to_bottom: i32;
    let mut bottom_to_top: i32;
    
    for i in 0..size {
        left_to_right = -1;
        right_to_left = -1;
        top_to_bottom = -1;
        bottom_to_top = -1;
        for j in 0..size {
            tmp = data[i * size + j];
            if tmp > left_to_right { 
                chosen[i * size + j] = 1;
                left_to_right = tmp;
            }

            tmp = data[(i + 1)*size - (j + 1)];
            if tmp > right_to_left { 
                chosen[(i + 1)*size - (j + 1)] = 1;
                right_to_left = tmp;
            }

            tmp = data[j * size + i];
            if tmp > top_to_bottom { 
                chosen[j * size + i] = 1;
                top_to_bottom = tmp;
            }

            tmp = data[(size - j - 1)*size + i];
            if tmp > bottom_to_top { 
                chosen[(size - j - 1)*size + i] = 1;
                bottom_to_top = tmp;
            }
        }

    }

    return chosen.iter().sum();
}

fn part_2(data: &Vec<i32>) -> i32 {
    let mut view: Vec<i32> = vec![0; data.len()];
    let size = if data.len() > 25 {99} else {5};
    let mut left: i32;
    let mut right: i32;
    let mut up: i32;
    let mut down: i32;
    
    for i in 0..data.len() {
        left = 1;
        right = 1;
        up = 1;
        down = 1;

        for j in 1..(i % size) {
            if data[i - j] < data[i] {
                left += 1;
            }
            else {
                break;
            }
        }

        for j in 1..(size - 1 - (i % size)) {
            if data[i + j] < data[i] {
                right += 1;
            }
            else {
                break;
            }
        }

        for j in 1..(i / size) {
            if data[i - j * size] < data[i] {
                up += 1;
            }
            else {
                break;
            }
        }

        for j in 1..(size - 1 - (i / size)) {
            if data[i + j * size] < data[i] {
                down += 1;
            }
            else {
                break;
            }
        }
        
        view[i] = left * right * up * down;
    }
    
    for i in 0..size {
        view[i] = 0;
        view[data.len() - 1 - i] = 0;
    }

    for i in 0..size {
        view[i * size] = 0;
        view[data.len() - 1 - i * size] = 0;
    }

    return view.iter().max().unwrap().clone();
}

fn get_data(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}