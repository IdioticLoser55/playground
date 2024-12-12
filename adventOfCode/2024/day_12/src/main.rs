use std::fs;
use std::time;
use std::ops::{Add, Sub};

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

    // Breadth First Queue
    let mut bfs_queue = Vec::new();
    let mut region_indicies = Vec::new();

    let mut p1 = 0;
    let mut p2 = 0;

    // loop through the entire grid.
    for (i, &plot) in grid.iter().enumerate() {
        // Check that haven't been here already.
        if visit_count[i] > 0 {continue};


        let pos = Point::from_idx(i, &bounds);

        // push current pos to queue, with no dir_flag.
        bfs_queue.push((pos, 0));
        region_indicies.clear();

        // loop until the queue is empty.
        while let Some((pos, dir_flag)) = bfs_queue.pop() {
            if !pos.is_within_bounds(&bounds) {
                continue;
            }

            let i = (pos.y * bounds.x + pos.x) as usize;

            // check if part of this region.
            if grid[i] != plot {
                continue;
            }

            // count how many times visited. / How many neighbours in this region.
            // 4 - neighbours = sides exposed.
            visit_count[i] += 1;
            // keep track of the direction from which the neighbours came.
            visited_from[i] |= dir_flag;

            // if already been visited then don't need to search neighbours.
            if visit_count[i] > 1 {
                continue;
            }

            // add current to region.
            region_indicies.push(i);

            // add neighbours to the queue to be searched.
            for (dir_idx, dir) in DIRS.iter().enumerate() {
                bfs_queue.push((dir + &pos, 1 << dir_idx))
            }
            
        }

        // Account for the initial visit from nowhere.
        visit_count[i] -= 1;

        // number of spaces visited in the region.
        let area = region_indicies.len();

        // visit count keeps track of neighbours in the same region.
        // 4 - neighbours = sides exposed.
        // Sum exposed sides to get perimiter.
        let perimeter: usize = region_indicies
            .iter()
            .map(|&idx| 4 - visit_count[idx])
            .sum();

        // Work back from perimeter to get sides.
        let mut sides = perimeter;
        // if area is 1 sides and perimeter are the same.
        if area != 1 {

            // search left to right, then top to bottom.
            // Pass in the flags for the opposite directions.
            // DIRS[0] = RIGHT VEC; 4 = DOWN FLAG; 8 = UP FLAG;
            // DIRS[3] = DOWN VEC; 1 = RIGHT FLAG; 2 = LEFT FLAG;
            for (dir, dir_flags) in [(DIRS[0], 4 | 8), (DIRS[3], 1 | 2)] {
                // for each point in the region.
                for &i in region_indicies.as_slice() {
                    // checks to see if visited from the flag directions.
                    if visited_from[i] & dir_flags == dir_flags {
                        continue;
                    }

                    // works out the coordinates of the adjacent pos.
                    let next_pos = &Point::from_idx(i, &bounds) + &dir;
                    if !next_pos.is_within_bounds(&bounds) {
                        continue;
                    }

                    // index of adjacent pos.
                    let next_i = next_pos.to_idx(&bounds);
                    // is index part of region.
                    if !region_indicies.contains(&next_i) {
                        continue;
                    }

                    // Or the two spaces. To find all the directions they were visited from.
                    // Apply NOT. To get all the directions they weren't visited from.
                    // Mask with dir_flags for the current directions being tested.
                    // If neither side was visited from one of the directions being tested
                    // then we can subtract 1 from sides/perimeter because they are adjacent.
                    // Because dir_flags contains the two opposite directions,
                    // we check both directions at the same time. And then count the number of ones
                    // in the result.
                    let bin_op = !(visited_from[i] | visited_from[next_i]) & dir_flags;
                    sides -= (bin_op).count_ones() as usize;
                }

            }
        }


        p1 += area * perimeter;
        p2 += area * sides;
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}
