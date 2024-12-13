use std::fs;
use std::time;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    let offset = Point{x: 10_000_000_000_000i64, y: 10_000_000_000_000i64};
    
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
    // let a_solve = (p.x * b.y - p.y * b.x) / (a.x * b.y - a.y * b.x);
    // let b_solve = (a.x * p.y - a.y * p.x) / (a.x * b.y - a.y * b.x);


    // Because we only want integer solutions and we only have one div per equation.
    // We can solve this without using float points.
    // Solve the top and bottom of a without dividing.
    // Then check top % bottom.
    // If it's zero you have an integer solution.
    // If not you have a fractional solution.
    // Then repeat for b.
    // And just take the division of the two parts.

    let a_top = p.x * b.y - p.y * b.x;
    let a_bottom = a.x * b.y - a.y * b.x;

    if a_top % a_bottom != 0 {
        return None;
    }


    let b_top = a.x * p.y - a.y * p.x;
    let b_bottom = a.x * b.y - a.y * b.x;

    if b_top % b_bottom != 0 {
        return None;
    }

    return Some(((a_top / a_bottom) as usize, (b_top / b_bottom) as usize))
}

fn parse_line(line: &[u8]) -> Point {
    let xs = line.iter().position(|&c| c == b'X').unwrap() + 2;
    let x = line[xs..].iter().map_while(|&c| if c.is_ascii_digit() {Some(parse_digit(c))} else {None}).reduce(|acc, el| acc * 10 + el).unwrap();
    let ys = line.len() - line.iter().rev().position(|&c| c == b'Y').unwrap() + 1;
    let y = line[ys..].iter().map_while(|&c| if c.is_ascii_digit() {Some(parse_digit(c))} else {None}).reduce(|acc, el| acc * 10 + el).unwrap();

    Point{x: x as i64, y: y as i64}
}

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
