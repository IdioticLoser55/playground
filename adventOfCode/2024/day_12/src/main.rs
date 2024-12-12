use std::fs;
use std::time;
use std::ops::{Add, Sub};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

const DIRS: [Point; 4] = [
    Point{x: 1, y: 0},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 0, y: -1},
];

fn main() {
    let file_path = "test2.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("ME: \n{}\n", bench(&input, me));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn me(input: &str) -> String {
    let lines: Vec<_> = input.trim().lines().collect();

    let grid: Vec<_> = lines
        .iter()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect();

    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};

    let mut visit_count: Vec<usize> = vec![0; grid.len()];
    visit_count.fill(0);

    let mut bfs_queue = Vec::new();
    let mut region_idx = Vec::new();

    let mut p1 = 0;

    for (i, &plot) in grid.iter().enumerate() {
        // println!("FOR: i: {i}, p: {plot}");
        if visit_count[i] > 0 {continue};

        let pos = Point::from_idx(i, &bounds);
        // println!("POINT: {:?}", pos);
        bfs_queue.push(pos);
        region_idx.clear();

        while let Some(pos) = bfs_queue.pop() {
            // println!("Loop Pos: {:?}", pos);

            if !pos.is_within_bounds(&bounds) {
                // println!("out of bounds");
                continue;
            }

            let i = (pos.y * bounds.x + pos.x) as usize;
            // println!("i: {i}");
            if grid[i] != plot {
                // println!("Not part of the region.");
                continue;
            }


            visit_count[i] += 1;
            if visit_count[i] > 1 {
                // println!("Already been");
                continue;
            }

            region_idx.push(i);
            for dir in DIRS.iter() {
                bfs_queue.push(dir + &pos)
            }
            
            // println!("Queue: {:?}", bfs_queue);
            // println!("Visit_count: {:?}", visit_count);
        }

        // account for initial. Non / self visit.
        visit_count[i] -= 1;
        // println!("Visit_count: {:?}", visit_count);

        let area = region_idx.len();
        let perimeter: usize = region_idx
            .iter()
            .map(|&idx| 4 - visit_count[idx])
            .sum();

        p1 += area * perimeter;
        // println!("I: {i}, PLOT: {plot}, area: {area}, Perim: {perimeter}");
    }

    println!("Part 1: {p1}");



    "".to_string()
}
