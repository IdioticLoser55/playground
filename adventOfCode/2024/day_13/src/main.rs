use std::fs;
use std::time;
use std::ops::{Add, Sub};
use std::slice::Iter;

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
    let mut p1 = 0;
    let mut p2 = 0;
    let offset = Point{x: 10_000_000_000_000i64, y: 10_000_000_000_000i64};

    let mut input_iter = input.as_bytes().iter();

    loop {
        let a = capture_next_point(&mut input_iter);
        if a.is_none() {
            break;
        }
        let a = a.unwrap();

        let b = capture_next_point(&mut input_iter).unwrap();
        let p = capture_next_point(&mut input_iter).unwrap();

        if let Some((a, b)) = solve(&a, &b, &p) {
            p1 += a * 3 + b;
        }

        if let Some((a, b)) = solve(&a, &b, &(&p + &offset)) {
            p2 += a * 3 + b;
        }
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn capture_next_point(input_iter: &mut Iter<u8>) -> Option<Point> {
    if advance_to_target(input_iter, b'X').is_none() {
        return None;
    }
    input_iter.next();
    let x = extract_number(input_iter);

    advance_to_target(input_iter, b'Y');
    input_iter.next();
    let y = extract_number(input_iter);

    Some(Point{x: x as i64, y: y as i64})
}

fn advance_to_target(input_iter: &mut Iter<u8>, target: u8) -> Option<()> {
    while let Some(char) = input_iter.next() {
        if *char == target {
            return Some(());
        }
    }

    None
}

fn extract_number(input_iter: &mut Iter<u8>) -> usize {
    let mut x = 0;
    while let Some(char) = input_iter.next() {
        if char.is_ascii_digit() {
            x = x * 10 + parse_digit(*char);
        } else {
            break;
        }
    }

    x
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

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
