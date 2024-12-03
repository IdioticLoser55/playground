use std::fs;
use std::time;
use std::collections::HashMap;

fn main() {
    bench(part1);
    bench(part2);
}

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut iter = line
        .split_whitespace()
        .into_iter()
        .map(|el| el.parse::<i32>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part1() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = contents
        // split the text into lines.
        .lines()
        .into_iter()
        // parse each line. Convert to a tuple for each side.
        .map(|l| parse_line(l))
        // fold tuples together into vectors.
        .unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    let distance = list1.iter().zip(list2).map(|(a, b)| (a - b).abs()).sum::<i32>();

    println!("{:#?}", distance);
}

fn part2() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    let (list1, list2): (Vec<i32>, Vec<i32>) = contents
        // split the text into lines.
        .lines()
        .into_iter()
        // parse each line. Convert to a tuple for each side.
        .map(|l| parse_line(l))
        // fold tuples together into vectors.
        .fold(
            (Vec::new(), Vec::new()),
            |(mut list1, mut list2), (a, b)| {
                list1.push(a);
                list2.push(b);
                (list1, list2)
            },
        );

    // let sim_score = list1.iter().fold(0, |acc, i| {
    //     acc + i * list2.iter().filter(|&n| n == i).count() as i32
    // });

    let mut count = HashMap::with_capacity(list1.len());
    for &e in &list2 {
        *count.entry(e).or_insert(0) += 1
    }


    let sim_score: i32 = list1.iter().map(|&a| a * count.get(&a).unwrap_or(&0)).sum(); println!("{sim_score}");
}
