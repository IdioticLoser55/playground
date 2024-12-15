use std::fs;
use std::ops::{AddAssign, Mul};
use std::time;
use std::ops::{Add, Sub, MulAssign};
use std::slice::Iter;

const DIRS: [Point; 4] = [
    Point{x: 1, y: 0},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 0, y: -1},
];

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y
    }
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

impl MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<i64> for &Point{
    type Output = Point;

    fn mul(self, rhs: i64) -> Point {
        Point{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Point {
    fn is_within_bounds(&self, bounds: &Self) -> bool {
        if self.x >= bounds.x || self.x < 0 || self.y >= bounds.y || self.y < 0 {
            false
        } else {
            true
        }
    }

    fn to_idx(&self, bounds: &Self) -> usize {
        (self.y * bounds.x + self.x) as usize
    }

    fn from_idx(idx: usize, bounds: &Self) -> Self {
        Point{x: idx as i64 % bounds.x, y: idx as i64 / bounds.x}
    }
}

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("FILE NOT FOUND");

    println!("Part 1: \n{}\n", bench(&input, part_1));
    println!("Part 2: \n{}\n", bench(&input, part_2));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn part_1(input: &str) -> String {
    let seconds_to_wait = 100;
    // let width = 11;
    // let height = 7;
    let width = 101;
    let height = 103;

    let mid_width = width / 2;
    let mid_height = height / 2;


    let mut input_iter = input.as_bytes().iter();
    let mut quadrant_counts = [0; 4];

    loop {
        let p = capture_next_point(&mut input_iter);
        if p.is_none() {
            break;
        }

        let mut p = p.unwrap();
        let mut v = capture_next_point(&mut input_iter).unwrap();
        
        v  *= seconds_to_wait;
        p += &v;

        p.x = p.x % width;
        p.y = p.y % height;

        if p.x < 0 {p.x = width + p.x};
        if p.y < 0 {p.y = height + p.y};


        if p.x < mid_width {
            if p.y < mid_height {
                quadrant_counts[0] += 1;
            } else if p.y > mid_height {
                quadrant_counts[2] += 1;
            }
        } else if p.x > mid_width {
            if p.y < mid_height {
                quadrant_counts[1] += 1;
            } else if p.y > mid_height {
                quadrant_counts[3] += 1;
            }
        }
    }

    let p1: usize = quadrant_counts.iter().product();

    format!("Part 1: {p1}")
}

fn part_2(input: &str) -> String {
    // let width = 11;
    // let height = 7;
    let width = 101_i64;
    let height = 103_i64;
    let bounds = Point{x: width, y: height};

    let max_time = width * height;

    let mut input_iter = input.as_bytes().iter();
    let mut robots: Vec<(Point, Point)> = Vec::new();

    loop {
        let p = capture_next_point(&mut input_iter);
        if p.is_none() {
            break;
        }

        let p = p.unwrap();
        let v = capture_next_point(&mut input_iter).unwrap();

        robots.push((p, v));
    }

    let mut max_region_count = 0;
    let mut max_region_time = 0;
    let mut indices: Vec<Point> = Vec::with_capacity(robots.len());
    let mut bfs_queue: Vec<Point> = Vec::new();
    let mut grid = vec![0; (width * height) as usize];
    let mut visited = vec![false; (width * height) as usize];

    for i in 1..=max_time {
        indices.clear();
        visited.fill(false);
        grid.fill(0);

        for (p, v) in robots.iter() {

            let mut v = v.clone();
            v  *= i as i64;
            let mut p = p + &v;

            p.x = p.x % width;
            p.y = p.y % height;

            if p.x < 0 {p.x = width + p.x};
            if p.y < 0 {p.y = height + p.y};

            grid[p.to_idx(&bounds)] += 1;
            indices.push(p);
        }

        for p in indices.as_slice() {
            bfs_queue.push(*p);

            let mut region_count = 0;
            while let Some(p) = bfs_queue.pop() {
                if !p.is_within_bounds(&bounds) {
                    continue;
                }

                let i = p.to_idx(&bounds);
                if grid[i] == 0 {
                    continue;
                }

                if visited[i] {
                    continue;
                }

                visited[i] = true;
                region_count += 1;


                for dir in DIRS.iter() {
                    bfs_queue.push(dir + &p);
                }
            }

            if region_count > max_region_count {
                max_region_count = region_count;
                max_region_time = i;
            }
        }
    }

    // // To print the Tree.
    // grid.fill(0);
    // for (p, v) in robots.iter() {

    //     let mut v = v.clone();
    //     v  *= max_region_time as i64;
    //     let mut p = p + &v;

    //     p.x = p.x % width;
    //     p.y = p.y % height;

    //     if p.x < 0 {p.x = width + p.x};
    //     if p.y < 0 {p.y = height + p.y};

    //     grid[p.to_idx(&bounds)] += 1;
    // }

    // grid
    //     .chunks(width as usize)
    //     .map(|line| String::from_utf8( line.iter().map(|num| if *num > 0 {b'#'} else {b'.'}).collect::<Vec<_>>()).unwrap())
    //     .for_each(|line| println!("{line}"));



    format!("Max Region Time: {max_region_time}, Max Region: {max_region_count}")
}

fn capture_next_point(input_iter: &mut Iter<u8>) -> Option<Point> {
    if advance_to_target(input_iter, b'=').is_none() {
        return None;
    }

    let x = extract_number(input_iter);
    let y = extract_number(input_iter);

    Some(Point{x, y})
}

fn advance_to_target(input_iter: &mut Iter<u8>, target: u8) -> Option<()> {
    while let Some(char) = input_iter.next() {
        if *char == target {
            return Some(());
        }
    }

    None
}

fn extract_number(input_iter: &mut Iter<u8>) -> i64 {
    let mut x = 0;
    let mut negative = false;

    if let Some(char) = input_iter.next() {
        match char {
            b'+' => (),
            b'-' => negative = true,
            c if c.is_ascii_digit() => x = parse_digit(*c),
            _ => panic!("Not a valid number"),
        }
    }

    while let Some(char) = input_iter.next() {
        if char.is_ascii_digit() {
            x = x * 10 + parse_digit(*char);
        } else {
            break;
        }
    }

    if negative {-(x as i64)} else {x as i64}
}

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
