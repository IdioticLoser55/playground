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
    let file_path = "input.txt";
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
    let mut visited_from: Vec<u8> = vec![0; grid.len()];

    let mut bfs_queue = Vec::new();
    let mut region_indicies = Vec::new();

    let mut p1 = 0;
    let mut p2 = 0;

    for (i, &plot) in grid.iter().enumerate() {
        // println!("FOR: i: {i}, p: {plot}");
        if visit_count[i] > 0 {continue};

        let pos = Point::from_idx(i, &bounds);
        // println!("POINT: {:?}", pos);
        bfs_queue.push((pos, 0));
        region_indicies.clear();

        while let Some((pos, dir_flag)) = bfs_queue.pop() {
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
            visited_from[i] |= dir_flag;
            if visit_count[i] > 1 {
                // println!("Already been");
                continue;
            }

            region_indicies.push(i);
            for (dir_idx, dir) in DIRS.iter().enumerate() {
                bfs_queue.push((dir + &pos, 1 << dir_idx))
            }
            
            // println!("Queue: {:?}", bfs_queue);
            // println!("Visit_count: {:?}", visit_count);
        }

        // account for initial. Non / self visit.
        visit_count[i] -= 1;
        // println!("Visit_count: {:?}", visit_count);

        let area = region_indicies.len();
        let perimeter: usize = region_indicies
            .iter()
            .map(|&idx| 4 - visit_count[idx])
            .sum();

        let mut sides = perimeter;
        // const DIRS: [Point; 4] = [
        //     Point{x: 1, y: 0},       1
        //     Point{x: -1, y: 0},      2
        //     Point{x: 0, y: 1},       4
        //     Point{x: 0, y: -1},      8
        // ];

        for (dir, dir_flags) in [(DIRS[0], 4 | 8), (DIRS[3], 1 | 2)] {
            // println!("DIR: {:?}, flags: {:#06b}", dir, dir_flags);
            for &i in region_indicies.as_slice() {
                // println!("I: {i}, visited_from[i]: {:#06b}", visited_from[i]);

                if visited_from[i] & dir_flags == dir_flags {
                    // println!("Already been visited from this dir.");
                    continue;
                }

                let next_pos = &Point::from_idx(i, &bounds) + &dir;
                if !next_pos.is_within_bounds(&bounds) {
                    // println!("Not in bounds.");
                    continue;
                }

                let next_i = next_pos.to_idx(&bounds);
                // println!("Next i: {next_i}");
                if !region_indicies.contains(&next_i) {
                    // println!("Not in region");
                    continue;
                }

                // println!("Sides: {sides}");
                // println!("visited_from[i]:  {:#06b}", visited_from[i]);
                // println!("visited_from[ni]: {:#06b}", visited_from[next_i]);
                // println!("dir_flags:        {:#06b}", dir_flags);
                let bin_op = !(visited_from[i] | visited_from[next_i]) & dir_flags;
                // println!("bin operation:    {:#06b}", bin_op);
                sides -= (bin_op).count_ones() as usize;
                // println!("Sides: {sides}");
            }
        }

        p1 += area * perimeter;
        p2 += area * sides;
        // println!("Visited From: {:?}", visited_from);
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}
