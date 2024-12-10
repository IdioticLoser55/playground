use std::fs;
use std::time;
use std::ops::{Add, Sub};
use std::collections::HashSet;


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

    println!("ME: {}", bench(&input, me));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn me(input: &str) -> String {
    // split into lines. needed here instead of topology so I can get number of lines.
    let lines: Vec<&[u8]> = input.as_bytes().trim_ascii().split(|&c| c == b'\n').collect();

    // parse lines.
    let topology: Vec<usize> = lines
        .iter()
        .flat_map(|line| {
            line.iter()
                .map(|&c| parse_digit(c))
        })
        .collect();

    // bounds of grid.
    let bounds = Point{x: (topology.len() / lines.len()) as i64, y: lines.len() as i64};

    // find all the starting points.
    let trailheads: Vec<usize> = topology
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == 0 {Some(i)} else {None}
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
    topology: &Vec<usize>, 
    map_of_nines: &mut Vec<Option<(HashSet<usize>, usize)>>,
    bounds: &Point,
    point: Point,
    prev_top: Option<usize>,
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

    if curr_top == 9 {
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
