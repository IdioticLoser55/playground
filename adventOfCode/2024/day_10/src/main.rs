use std::fs;
use std::time;
use std::ops::{Add, Sub};
use std::collections::VecDeque;
use hashbrown::HashSet;



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
}
const POINTS: [Point; 4] = [
    Point{x: 1, y: 0},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 0, y: -1},
];

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("ME: \n{}\n", bench(&input, me));
    println!("BFS: \n{}\n", bench(&input, bfs));
    println!("SMTH: \n{}\n", bench(&input, something));
    println!("Me2: \n{}\n", bench(&input, me_attempt2));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn me_attempt2(input: &str) -> String {
    // had a go at improving something. And adding a nines map from my original attempt.
    // The recursive solution naturally keeps track of your path. But this doesn't so you'd have to
    // add it.
    // And either way all the new array / hashset creation just tanks perf.
    let lines = input.trim().lines().collect::<Vec<_>>();
    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let trailheads = grid
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == b'0' {
                Some(Point{x: i as i64 % bounds.x, y: i as i64 / bounds.x})
            } else {
                None
            }
        }).collect::<Vec<_>>();

    let mut total1 = 0;
    let mut total2 = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![false; grid.len()];
    for t in trailheads {
        queue.push_back(t);
        seen.fill(false);
        while let Some(pos) = queue.pop_front() {
            let i = (pos.y * bounds.x + pos.x) as usize;
            let c = grid[i];
            if c == b'9' {
                if !seen[i] {
                    total1 += 1;
                    seen[i] = true;
                }
                total2 += 1;
                continue;
            }
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = pos.x + dir.0;
                let ny = pos.y + dir.1;
                if nx >= 0 && ny >= 0 && nx < bounds.x && ny < bounds.y {
                    let d = grid[(ny * bounds.x + nx) as usize];
                    if d == c + 1 {
                        queue.push_back(Point{x: nx, y: ny});
                    }
                }
            }
        }
    }

    format!("Part 1: {total1}\nPart 2: {total2}")
}

fn something(input: &str) -> String {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut trailheads = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'0' {
                trailheads.push((x as i32, y as i32));
            }
        }
    }

    let mut total1 = 0;
    let mut total2 = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![false; grid.len()];
    for t in trailheads {
        queue.push_back(t);
        seen.fill(false);
        while let Some(pos) = queue.pop_front() {
            let i = pos.1 as usize * width + pos.0 as usize;
            let c = grid[i];
            if c == b'9' {
                if !seen[i] {
                    total1 += 1;
                    seen[i] = true;
                }
                total2 += 1;
                continue;
            }
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = pos.0 + dir.0;
                let ny = pos.1 + dir.1;
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let d = grid[ny as usize * width + nx as usize];
                    if d == c + 1 {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    format!("Part 1: {total1}\nPart 2: {total2}")
}

fn reachable_nines(g: &[&[u8]], r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut q = VecDeque::from([(r, c)]);
    let mut seen = Vec::new();
    while let Some((r, c)) = q.pop_front() {
        if g[r][c] == b'9' {
            seen.push((r, c));
            continue;
        }
        for (rr, cc) in [(r+1, c), (r-1, c), (r, c+1), (r, c-1)] {
            if *g.get(rr).and_then(|row| row.get(cc)).unwrap_or(&0) == g[r][c] + 1 {
                q.push_back((rr, cc));
            }
        }
    }
    seen
}

fn bfs(input: &str) -> String {
    let g = input.trim().lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let (mut p1, mut p2) = (0, 0);
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == b'0' {
                let seen = reachable_nines(&g, r, c);
                p1 += seen.iter().collect::<HashSet<_>>().len();
                p2 += seen.len();
            }
        }
    }
    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn me(input: &str) -> String {
    // split into lines. needed here instead of topology so I can get number of lines.
    let lines = input.trim().lines().collect::<Vec<_>>();

    // parse lines.
    let topology: Vec<u8> = lines
        .iter()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect();

    // bounds of grid.
    let bounds = Point{x: (topology.len() / lines.len()) as i64, y: lines.len() as i64};

    // find all the starting points.
    let trailheads: Vec<usize> = topology
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == b'0' {Some(i)} else {None}
        })
        .collect();

    // map to keep track of points and where you can get from here. Also the counts for part two.
    let mut map_of_nines: Vec<Option<(HashSet<usize>, usize)>> = vec![None; topology.len()];

    let mut p1 = 0;
    let mut p2 = 0;
    for head in trailheads {
        let (nines_found, nines_routes) = find_nines_count(
            &topology,
            &mut map_of_nines,
            &bounds,
            Point{x: head as i64 % bounds.x, y: head as i64 / bounds.x},
            None,
        ).unwrap();

        p1 += nines_found.len();
        p2 += nines_routes;
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn find_nines_count(
    topology: &Vec<u8>, 
    map_of_nines: &mut Vec<Option<(HashSet<usize>, usize)>>,
    bounds: &Point,
    point: Point,
    prev_top: Option<u8>,
) -> Option<(HashSet<usize>, usize)> {
    if !point.is_within_bounds(bounds) {
        // Not within bounds.
        return None;
    }

    let pos = usize::try_from(point.y * bounds.x + point.x).unwrap();
    let curr_top = topology[pos];

    if let Some(prev_top) = prev_top {
        if curr_top.cmp(&prev_top).is_le() || curr_top - prev_top != 1 {
            // gradient diff between here and last is not 1.
            return None;
        }
    }

    if curr_top == b'9' {
        // found the top;
        // return new hashset and route count.
        let mut set = HashSet::new();
        set.insert(pos);
        return Some((set, 1));
    }

    // check to see if already been to this square. Return previously worked out results.
    if let Some(nines_from_here) = &map_of_nines[pos] {
        return Some(nines_from_here.clone());
    }


    // Recursively try each direction.
    if let Some(results) = POINTS.iter()
        .filter_map(|delta| {
            find_nines_count(&topology, map_of_nines, &bounds, &point + delta, Some(curr_top))
        }).reduce(|(mut set, count), (in_set, in_count)| {
            // Combine the results from each direction.
            set.extend(in_set);
            (set, count + in_count)
        }) 
    {
        // update map and return.
        map_of_nines[pos] = Some(results.clone());
        Some(results)
    } else {
        // Everything returned None. Can't get anywhere from here.
        // So empty set. 
        //
        // No point creating and returning four empty sets earlier only to have to combine them
        // here.
        let results = (HashSet::new(), 0);
        map_of_nines[pos] = Some(results.clone());
        Some(results)
    }

}


fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
