use std::{fs, collections::HashMap, time::Instant};



fn main() {
    let overall_timer = Instant::now();
    let mut timer = Instant::now();
    let data = get_data("day3/resources/input.txt");
    let mut hashmap = HashMap::<u64, u8>::new();
    populate_hashmap_with_keys_and_priorities(&mut hashmap);
    let reading_time = timer.elapsed();
    
    timer = Instant::now();
    let part_1_ans = part_1(&hashmap, &data);
    let part_1_time = timer.elapsed();
    
    timer = Instant::now();
    let part_2_ans = part_2(&hashmap, &data);
    let part_2_time = timer.elapsed();
    let overall_time = overall_timer.elapsed();

    println!("The answer for part 1 is: {}", part_1_ans);
    println!("The answer for part 2 is: {}", part_2_ans);
    println!("The overall time is {:.2?}", overall_time);
    println!("The reading phase took {:.2?}, part 1 took {:.2?} and part 2 took {:.2?}", reading_time, part_1_time, part_2_time);
}

fn part_1(hashmap: &HashMap<u64, u8>, data: &Vec<String>) -> u32 {
    let mut first_comp: u64;
    let mut second_comp: u64;
    let mut total: u32 = 0;
    let mut line_split: (&str, &str);
    for line in data {
        line_split = line.split_at(line.len() / 2);

        first_comp = 0;
        for character in line_split.0.chars() {
            first_comp = first_comp | ((1 as u64) << convert_ascii_to_priority(&(character as u8)));
        }
        
        second_comp = 0;
        for character in line_split.1.chars() {
            second_comp = second_comp | ((1 as u64) << convert_ascii_to_priority(&(character as u8)));
        }


        total += *hashmap.get(&(first_comp & second_comp)).unwrap() as u32;
    }
    
    return total
}

fn part_2(hashmap: &HashMap<u64, u8>, data: &Vec<String>) -> u32 {
    let mut comps: [u64; 3] = [0, 0, 0];
    let mut total: u32 = 0;

    for i in (0..data.len()).step_by(3) {
        comps[0] = 0;
        for character in data[i].chars() {
            comps[0] = comps[0] | ((1 as u64) << convert_ascii_to_priority(&(character as u8)));
        }   
        
        comps[1] = 1;
        for character in data[i + 1].chars() {
            comps[1] = comps[1] | ((1 as u64) << convert_ascii_to_priority(&(character as u8)));
        }   

        comps[2] = 2;
        for character in data[i + 2].chars() {
            comps[2] = comps[2] | ((1 as u64) << convert_ascii_to_priority(&(character as u8)));
        }   
        
        total += *hashmap.get(&(comps[0] & comps[1] & comps[2])).unwrap() as u32;
    }
    
    return total;
}


fn convert_ascii_to_priority(ascii_char: &u8) -> u8 {
    match ascii_char {
        96.. => ascii_char - 96,
        _ => ascii_char - 38,
    }
}

fn populate_hashmap_with_keys_and_priorities(hashmap: &mut HashMap<u64, u8>){
    for i in 1..53 {
        hashmap.insert(2_u64.pow(i as u32), i);
    }
}

fn get_data(name: &str) -> Vec<String> {
    fs::read_to_string(name).unwrap()
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>()
}