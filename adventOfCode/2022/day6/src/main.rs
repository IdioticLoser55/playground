use std::{fs, time::Instant};

fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();

    let data = get_data("day6/input.txt");
    let reading_time = timer.elapsed();
    
    timer = Instant::now();
    let part_1_ans = part_1(&data);
    let part_1_time = timer.elapsed();
    
    timer = Instant::now();
    let part_2_ans = part_2(&data);
    let part_2_time = timer.elapsed();
    let overall_time = overall_timer.elapsed();

    timer = Instant::now();
    someone_else(&data);
    let someone_else_time = timer.elapsed();

    println!("The answer for part 1 is: {}", part_1_ans);
    println!("The answer for part 2 is: {}", part_2_ans);
    println!("The overall time is {:.2?}", overall_time);
    println!("The reading phase took {:.2?}, part 1 took {:.2?} and part 2 took {:.2?}", reading_time, part_1_time, part_2_time);
    println!("Someone else's solution that I found but don't fully understand: {:.2?}", someone_else_time);
}

fn part_1(data: &Vec<u32>) -> i32 {
    let size = 4;
    let mut count = size - 1;

    'outer: loop {
        count += 1;
        let mut reduce = data[count - size];
        for i in (count - size + 1)..count {
            reduce = match reduce & data[i] {
                0 => reduce | data[i],
                _ => continue 'outer,
            };
        }
        break 'outer;
    }
    
    count as i32
}

fn part_2(data: &Vec<u32>) -> i32 {
    let size = 14;
    let mut count = size - 1;

    'outer: loop {
        count += 1;
        let mut reduce = data[count - size];
        for i in (count - size + 1)..count {
            reduce = match reduce & data[i] {
                0 => reduce | data[i],
                _ => continue 'outer,
            };
        }
        break 'outer;
    }
    
    count as i32
}

// found someone elses solution. Uses XOR instead of AND.
// On first finding a letter the XOR adds it.
// If it finds it again it will be removed.
// To check for all different it counts the ones.
// If there are any duplicates the count won't be right.
// Letters are removed from the group/ count after they are passed.
fn someone_else(data: &Vec<u32>) -> i32 {
    let size = 14;
    let mut accum = 0u32;
    
    for (i, mask) in data.iter().enumerate() {
        accum ^= mask;
        if i >= size {
            accum ^= data[i - size];
            if accum.count_ones() as usize == size {
                return i as i32 + 1;
            }
        }
    }
    
    panic!("Marker not found");
}

fn get_data(name: &str) -> Vec<u32> {
    fs::read_to_string(name).unwrap()
        .trim()
        .chars()
        .map(|c| 1u32 << ((c as u8) - 97))
        .collect::<Vec<u32>>()
}