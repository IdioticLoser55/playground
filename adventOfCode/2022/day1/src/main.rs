use advent_lib;
use std::str::Split;

fn main() {
    let text = advent_lib::read_file(&String::from("day1/resources/input.txt"));
    let elves: Split<&str> = text.split("\n\n");

    let mut max_count: usize = 0;
    let mut max2_count: usize = 0;
    let mut max3_count: usize = 0;
    let mut calorie_count: Vec<i32> = Vec::new();

    let mut count: usize = 0;
    for elf in elves {
        
        let snacks: Split<&str> = elf.split("\n");
        calorie_count.push(0);
        for snack in snacks {
            calorie_count[count] += snack.parse::<i32>().expect("snack is not a number");
        }
        
        //println!("count {}, calorie_count {}, max_count {}", count, calorie_count[count], max_count);
        if calorie_count[max_count] < calorie_count[count] {
            max3_count = max2_count;
            max2_count = max_count;
            max_count = count;
        }
        else if calorie_count[max2_count] < calorie_count[count] {
            max3_count = max2_count;
            max2_count = count;
        }
        else if calorie_count[max3_count] < calorie_count[count] {
            max3_count = count;
        }

        count += 1;
    }
    
    let sum = calorie_count[max_count] + calorie_count[max2_count] + calorie_count[max3_count];
    
    println!("{}", sum);
}
