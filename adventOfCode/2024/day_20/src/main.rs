use std::fmt::Binary;
use std::fs;
use std::time;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::ops::{AddAssign, Mul};
use std::ops::{Add, Sub};
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug)]
struct CheatHead {
    position: Point,
    score: usize,
    cheat_count: usize,
    cheat_start: Option<usize>,
}

// makes it so route head struct is sorted by the score.
impl Ord for CheatHead {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for CheatHead {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    let lines = input.trim().lines().collect::<Vec<_>>();

    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let start_pos = Point::from_idx(grid.iter().position(|c| *c == b'S').unwrap(), &bounds);

    let mut visited = vec![usize::MAX; grid.len()];
    let mut points_to_check: BinaryHeap<Reverse<RouteHead>> = BinaryHeap::new();

    find_path(
        &grid,
        start_pos,
        bounds,
        &mut visited,
        &mut points_to_check,
    );

    find_cheats(
        &grid,
        bounds,
        &visited,
    );


    "".to_string()
}

fn find_cheats(
    grid: &Vec<u8>,
    bounds: Point,
    visited: &Vec<usize>,
) {
    // DON'T NEED TO WALK PATH. Already have path. Just search visited for usize::MAX.
    // Can then just branch of from there.
    // Only search along walls.
    
    // start pos against end pos.
    let mut p1 = 0;
    let mut p2 = 0;
    let mut cheats: HashSet<usize> = HashSet::new();
    let mut points_to_check = BinaryHeap::new();
    let mut cheat_visited: Vec<usize> = vec![usize::MAX; visited.len()];

    let path_positions = visited
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v != usize::MAX {Some(i)} else {None});

    for start_pos in path_positions {
        cheat_visited.fill(usize::MAX);
        cheat_visited[start_pos] = 0;
        
        let point = Point::from_idx(start_pos, &bounds);
        for dir in DIRS{
            let npos = &point + &dir;

            points_to_check.push(Reverse(RouteHead{
                position: npos,
                score: 1,
            }))
        }

        while let Some(Reverse(route_head)) = points_to_check.pop() {
            let nscore = route_head.score + 1;
            if nscore > 20 {
                continue;
            }

            for dir in DIRS {
                let npos = &route_head.position + &dir;

                if !npos.is_within_bounds(&bounds) {
                    continue;
                }

                let ni = npos.to_idx(&bounds);
                if nscore >= cheat_visited[ni] {
                    continue;
                }
                cheat_visited[ni] = nscore;

                if grid[ni] != b'#' {
                    if visited[start_pos] + nscore < visited[ni] {
                        cheats.insert(ni);
                    }
                }
                points_to_check.push(Reverse(RouteHead{
                    position: npos,
                    score: nscore,
                }));
            }
        }

        for end_pos in cheats.drain() {
            let cheating_steps = cheat_visited[end_pos];
            let time_saved = visited[end_pos] - (visited[start_pos] + cheating_steps);

            if time_saved >= 100 {
                if cheating_steps == 2 {
                    p1 += 1;
                }

                p2 += 1;
            }
        }
    }

    println!("Part 1: {p1}\nPart 2: {p2}");

}

fn find_path(
    grid: &Vec<u8>,
    start_pos: Point,
    bounds: Point,
    visited: &mut Vec<usize>,
    points_to_check: &mut BinaryHeap<Reverse<RouteHead>>,
) {
    visited[start_pos.to_idx(&bounds)] = 0;

    points_to_check.push(Reverse(RouteHead{
        position: start_pos,
        score: 0,
    }));

    while let Some(Reverse(route_head)) = points_to_check.pop() {
        // add the cardinal directions.
        for dir in DIRS {
            let npos = &route_head.position + &dir;

            if !npos.is_within_bounds(&bounds) {
                continue;
            }

            let ni = npos.to_idx(&bounds);
            if grid[ni] == b'#' {
                    continue;
            }

            let nscore = route_head.score + 1;
            if nscore >= visited[ni] {
                continue;
            }
            visited[ni] = nscore;

            points_to_check.push(
                Reverse(RouteHead{
                    position: npos,
                    score: nscore,
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
