use std::fs;
use std::time;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
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
    println!("my_attempt 2: \n{}\n", bench(&input, my_attempt2));
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
    )
}

fn my_attempt2(input: &str) -> String {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let start_pos = Point::from_idx(grid.iter().position(|c| *c == b'S').unwrap(), &bounds);

    let mut visited = vec![usize::MAX; grid.len()];

    let path = find_path2(
        &grid,
        start_pos,
        bounds,
        &mut visited,
    );

    find_cheats2(
        &path,
        bounds,
        &visited,
    )
}

fn find_cheats2(
    path: &Vec<Point>,
    bounds: Point,
    visited: &Vec<usize>,
) -> String {
    let mut p1 = 0;
    let mut p2 = 0;

    //for a time saving of at least 100, we need to be comparing against positions at least 100
    //further along.
    for (i, start_pos) in path[..path.len() - 100].iter().enumerate() {
        for end_pos in &path[i + 100..] {
            
            let start = start_pos.to_idx(&bounds);
            let end = end_pos.to_idx(&bounds);

            let diff = start_pos - end_pos;
            let dist = (diff.x.abs() + diff.y.abs()) as usize;

            if dist > 20 {
                continue;
            }

            let cheat_time = visited[start] + dist;
            if cheat_time >= visited[end] {
                continue;
            }

            let saved_time = visited[end] - cheat_time;
            if saved_time >= 100 {
                if dist == 2 {
                    p1 += 1;
                }
                p2 += 1;
            }
        }
    }

    format!("Part1: {p1}\nPart2: {p2}")
}

fn find_cheats(
    grid: &Vec<u8>,
    bounds: Point,
    visited: &Vec<usize>,
) -> String {
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

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn find_path2(
    grid: &Vec<u8>,
    start_pos: Point,
    bounds: Point,
    visited: &mut Vec<usize>,
) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();

    let mut step_count = 0;
    let mut current_pos = start_pos.clone();
    let mut dir = DIRS.iter().find(|dir| grid[(&current_pos + dir).to_idx(&bounds)] != b'#').unwrap().clone();

    loop {
        path.push(current_pos);
        visited[current_pos.to_idx(&bounds)] = step_count;
        step_count += 1;

        let mut npos = &current_pos + &dir;
        if grid[npos.to_idx(&bounds)] == b'#' {
            let d = [Point{x: dir.y, y: -dir.x}, Point{x: -dir.y, y: dir.x}]
                .into_iter()
                .find(|dir| grid[(&current_pos + dir).to_idx(&bounds)] != b'#');

            if let Some(d) = d {
                dir = d;
            } else {
                break;
            }

            npos = &current_pos + &dir;
        }

        current_pos = npos;
    }

    path
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
