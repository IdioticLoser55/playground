use std::fs;
use std::time;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn within_bounds(&self, width: i64, height: i64) -> bool {
        if self.x >= width || self.x < 0 || self.y >= height || self.y < 0 {
            false
        } else {
            true
        }
    }
}

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("My_Mess: \n{}\n", bench(&input, my_mess));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn my_mess(input: &str) -> String {
    let lines = input
        .trim_ascii()
        .as_bytes()
        .split(|c| *c == b'\n')
        .collect_vec();

    let (width, height): (i64, i64) = (
        (lines[0].len()).try_into().unwrap(), 
        (lines.len()).try_into().unwrap()
    );

    let mut hash_map: HashMap<u8, Vec<Point>> = HashMap::new();
    let mut p1_grid: Vec<u8> = Vec::with_capacity((width * height).try_into().unwrap());

    lines
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .iter()
                .enumerate()
                .for_each(|(x, &c)| {
                    p1_grid.push(c);
                    if c != b'.' {
                        hash_map
                            .entry(c)
                            .or_insert_with(Vec::new)
                            .push(Point{
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            })
                    }
                })
        });
    let mut p2_grid = p1_grid.clone();

    // println!("HashMap: {:#?}", hash_map);
    // print_grid(&grid, width);

    for antenae in hash_map.values() {
        // println!("Antenae: {:?}", antenae);
        let product = antenae.iter().combinations(2);
        // println!("Product: {:?}", product);
        for pair in product {
            let (p1, p2) = (pair[0], pair[1]);
            p2_grid[usize::try_from(p1.y * width + p1.x).unwrap()] = b'#';
            p2_grid[usize::try_from(p2.y * width + p2.x).unwrap()] = b'#';

            let diff = p2 - p1;

            let mut a1 = p2 + &diff;
            if a1.within_bounds(width, height) {
                p1_grid[usize::try_from(a1.y * width + a1.x).unwrap()] = b'#';
                p2_grid[usize::try_from(a1.y * width + a1.x).unwrap()] = b'#';

                a1 = &a1 + &diff;
                while a1.within_bounds(width, height) {
                    p2_grid[usize::try_from(a1.y * width + a1.x).unwrap()] = b'#';
                    a1 = &a1 + &diff;
                }
            }


            let mut a2 = p1 - &diff;
            if a2.within_bounds(width, height) {
                p1_grid[usize::try_from(a2.y * width + a2.x).unwrap()] = b'#';
                p2_grid[usize::try_from(a2.y * width + a2.x).unwrap()] = b'#';

                a2 = &a2 - &diff;
                while a2.within_bounds(width, height) {
                    p2_grid[usize::try_from(a2.y * width + a2.x).unwrap()] = b'#';
                    a2 = &a2 - &diff;
                }
            }

        }
    }

    let p1 = p1_grid.iter().filter(|&&c| c == b'#').count();
    let p2 = p2_grid.iter().filter(|&&c| c == b'#').count();

    format!("Part 1: {}\nPart 2: {}", p1, p2)
}

fn print_grid(grid: &Vec<u8>, width: i64) {
    for line in grid
        .chunks(width.try_into().unwrap())
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>() {
        println!("{}", line);
    }
}
