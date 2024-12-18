use std::fs;
use std::time;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::ops::{AddAssign, Mul};
use std::ops::{Add, Sub};
use std::slice::Iter;


#[derive(PartialEq, Eq, Debug)]
struct RouteHead {
    position: Point,
    score: usize,
}

// makes it so route head struct is sorted by the score.
impl Ord for RouteHead {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for RouteHead {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [Point; 4] = [
    Point{x: 1, y: 0},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 0, y: -1},
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Mul<&Point> for i64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        Point{
            x: self * rhs.x,
            y: self * rhs.y
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

    println!("my_attempt: \n{}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn my_attempt(input: &str) -> String {
    let start_pos = Point{x: 0, y: 0};
    // let end_pos = Point{x: 6, y: 6};
    // let bounds = Point{x: 7, y: 7};
    // let limit = 12;
    let end_pos = Point{x: 70, y: 70};
    let bounds = Point{x: 71, y: 71};
    let limit = 1024;

    let mut points = Vec::new();
    let mut input_iter = input.trim().as_bytes().iter();
    let mut grid = vec![usize::MAX; (bounds.x * bounds.y) as usize];
    let mut visited = vec![usize::MAX; (bounds.x * bounds.y) as usize];
    let mut points_to_check: BinaryHeap<Reverse<RouteHead>> = BinaryHeap::new();

    // parse input.
    while let Some(x) = parse_while_number(&mut input_iter) {
        let y = parse_while_number(&mut input_iter).unwrap();
        let point = Point{x: x as i64 , y: y as i64};
        points.push(point);
    }

    // insert walls into grid, and store the time they occur.
    points.iter().enumerate().for_each(|(i, p)| grid[p.to_idx(&bounds)] = i);

    // find shortes path to end.
    find_path(&grid, start_pos, end_pos, bounds, &mut visited, &mut points_to_check, limit);
    let p1 = visited[end_pos.to_idx(&bounds)];


    let mut left = 0;
    let mut right = *grid.iter().filter(|x| **x != usize::MAX).max().unwrap();
    let mut mid = (left + right) / 2;

    // perform binary search until left >= mid.
    while left < mid {
        // clear visited.
        visited.fill(usize::MAX);
        // traverse current grid.
        find_path(&grid, start_pos, end_pos, bounds, &mut visited, &mut points_to_check, mid);

        // if end hasn't been visited it will still have default val of MAX.
        if visited[end_pos.to_idx(&bounds)] != usize::MAX {
            // path is valid. Bring left forwards to current mid.
            left = mid;
            mid = (left + right) / 2;

        } else {
            // invalid path. Need to lower search area.
            right = mid;
            mid = (left + right) / 2;
        }
    }

    let p2 = format!("{},{}", &points[left].x, &points[left].y);

    format!("Part 1: {p1}\nPart 2: {p2}")
}


fn find_path(
    grid: &Vec<usize>,
    start_pos: Point,
    end_pos: Point,
    bounds: Point,
    visited: &mut Vec<usize>,
    points_to_check: &mut BinaryHeap<Reverse<RouteHead>>,
    time_period: usize,
) {
    // init with start pos.
    points_to_check.push(Reverse(RouteHead{
        position: start_pos,
        score: 0
    }));

    while let Some(Reverse(route_head)) = points_to_check.pop() {
        if !route_head.position.is_within_bounds(&bounds) {
            continue;
        }

        let i = route_head.position.to_idx(&bounds);
        // check for obstacle in current time period.
        if grid[i] < time_period {
            continue;
        }

        // check if already visited and score when did so.
        if route_head.score >= visited[i] {
            continue;
        }
        visited[i] = route_head.score;

        // check if reached the end.
        if route_head.position == end_pos {
            break;
        }

        // add the cardinal directions.
        for dir in DIRS {
            points_to_check.push(
                Reverse(RouteHead{
                    position: &route_head.position + &dir,
                    score: route_head.score + 1,
                })
            )
        }
    }

    points_to_check.clear();
}


fn parse_while_number(number_iter: &mut Iter<u8>) -> Option<usize> {
    let char = number_iter.next();
    if char.is_none_or(|c| !c.is_ascii_digit()) {
        return None;
    }

    let mut acc = parse_digit(*char.unwrap());

    while let Some(char) = number_iter.next() {
        if char.is_ascii_digit() {
            acc = acc * 10 + parse_digit(*char);
        } else {
            return Some(acc);
        }
    }

    Some(acc)
}

fn parse_number(number: &[u8]) -> usize {
    number.iter()
        .fold(0, |acc, digit| {
            acc * 10 + parse_digit(*digit)
        })
}

fn parse_digit(number: u8) -> usize {
    (number - b'0') as usize
}

fn print_grid(grid: &Vec<u8>, width: usize) {
    for line in grid
        .chunks(width)
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>() {
        println!("{}", line);
    }
}
