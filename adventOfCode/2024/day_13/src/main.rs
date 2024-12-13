use std::fs;
use std::time;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
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

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("my_attempt: \n{}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn my_attempt(input: &str) -> String {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;
    let offset = Point{x: 10_000_000_000_000f64, y: 10_000_000_000_000f64};
    
    for chunk in lines.chunks(4) {
        let a = parse_line(chunk[0].as_bytes());
        let b = parse_line(chunk[1].as_bytes());
        let p = parse_line(chunk[2].as_bytes());

        if let Some((a, b)) = solve(&a, &b, &p) {
            p1 += a * 3 + b;
        }

        if let Some((a, b)) = solve(&a, &b, &(&p + &offset)) {
            p2 += a * 3 + b;
        }



    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn solve(a: &Point, b: &Point, p: &Point) -> Option<(usize, usize)> {
    // This is just simultaneous equations.
    // But you have to be careful when solving.
    // Don't walk through randomly. And instead try keep divisions to a minimum.
    //
    // Might have been easier to use Cramers method.
    let a_solve = (p.x * b.y - p.y * b.x) / (a.x * b.y - a.y * b.x);
    let b_solve = (a.x * p.y - a.y * p.x) / (a.x * b.y - a.y * b.x);

    if a_solve.fract() == 0.0 && b_solve.fract() == 0.0 {
        return Some((a_solve as usize, b_solve as usize));
    } else {
        return None;
    }
}

fn parse_line(line: &[u8]) -> Point {
    let xs = line.iter().position(|&c| c == b'X').unwrap() + 2;
    let x = line[xs..].iter().map_while(|&c| if c.is_ascii_digit() {Some(parse_digit(c))} else {None}).reduce(|acc, el| acc * 10 + el).unwrap();
    let ys = line.len() - line.iter().rev().position(|&c| c == b'Y').unwrap() + 1;
    let y = line[ys..].iter().map_while(|&c| if c.is_ascii_digit() {Some(parse_digit(c))} else {None}).reduce(|acc, el| acc * 10 + el).unwrap();

    Point{x: x as f64, y: y as f64}
}

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
