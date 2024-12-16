use std::fs;
use std::ops::{AddAssign, Mul};
use std::time;
use std::ops::{Add, Sub};
use std::collections::VecDeque;

const DIRS: [Point; 4] = [
    Point{x: 1, y: 0},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 0, y: -1},
];

#[derive(Debug, Clone, Copy, PartialEq)]
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

    println!("Part 1: \n{}\n", bench(&input, part_1));
    println!("Part 2: \n{}\n", bench(&input, part_2));
    println!("Better: \n{}\n", bench(&input, better));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn better(input: &str) -> String {
    let (map, moves) = input.trim().split_once("\n\n").unwrap();

    let lines = map.split("\n").collect::<Vec<_>>();
    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};


    let mut map = lines
        .iter()
        .flat_map(|line| {
            line.as_bytes()
        })
        .copied()
        .collect::<Vec<_>>();

    let fat_bounds = 2 * &bounds;
    let mut fat_map: Vec<u8> = Vec::with_capacity((fat_bounds.x * fat_bounds.y) as usize);
    map
        .iter()
        .for_each(|c| match c {
            b'#' => fat_map.extend(b"##"),
            b'O' => fat_map.extend(b"[]"),
            b'.' => fat_map.extend(b".."),
            b'@' => fat_map.extend(b"@."),
            _ => panic!(""),
        });

    let mut pos = map.iter().position(|c| *c == b'@').unwrap() as i64;
    // by getting rid of the robot symbol we don't have to keep track of it.
    map[pos as usize] = b'.';

    let mut fat_pos = fat_map.iter().position(|c| *c == b'@').unwrap() as i64;
    fat_map[fat_pos as usize] = b'.';

    let moves = moves
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied();

    let mut try_push: VecDeque<i64> = VecDeque::new();
    let mut to_push: Vec<i64> = Vec::new();

    for m in moves {
        // println!("\n{:?}, {:?}", String::from_utf8(vec![m]), fat_pos);
        // print_grid(&fat_map, fat_bounds.x);

        let (dir, fat_dir) = match m {
            b'>' => (1, 1),
            b'<' => (-1, -1),
            b'v' => (bounds.x, fat_bounds.x),
            b'^' => (-bounds.x, -fat_bounds.x),
            _ => panic!("ARGHHGHGHGHG"),
        };

        // advance map.
        part1_better(&mut map, &mut pos, dir);

        // advance fat map.
        part2_better(
            &mut try_push,
            &mut to_push,
            &mut fat_map, 
            &mut fat_pos,
            fat_dir,
        );
    }

    // println!("\n");
    // print_grid(&fat_map, fat_bounds.x);

    // Find and work out scores of the boxes.
    let mut p1 = 0;
    for (i, c) in map.iter().enumerate() {
        if *c == b'O' {
            let pos = Point::from_idx(i, &bounds);
            let score = 100 * pos.y + pos.x;
            // println!("i: {i}, Pos: {:?}, score: {score}", pos);
            p1 += score;
        }
        
    }

    let mut p2 = 0;
    for (i, c) in fat_map.iter().enumerate() {
        if *c == b'[' {
            let pos = Point::from_idx(i, &fat_bounds);
            let score = 100 * pos.y + pos.x;
            // println!("i: {i}, Pos: {:?}, score: {score}", pos);
            p2 += score;
        }
        
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn part1_better(
    map: &mut Vec<u8>,
    pos: &mut i64,
    dir: i64,
) {
    let mut i = *pos + dir;

    if map[i as usize] == b'.' {
        *pos += dir;
        return;
    }

    while map[i as usize] != b'.' && map[i as usize] != b'#' {
        i += dir;
    }

    if map[i as usize] == b'.' {
        map[i as usize] = b'O';
        *pos += dir;
        map[*pos as usize] = b'.';
    }
}

fn part2_better(
    try_push: &mut VecDeque<i64>,
    to_push: &mut Vec<i64>,
    map: &mut Vec<u8>,
    pos: &mut i64,
    dir: i64,
) {
    if dir.abs() == 1 {
        // If moving horizontal don't have to really deal with the two wide thing.
        // So can skip some steps.
        let mut i = *pos + dir;
        
        while map[i as usize] != b'.' && map[i as usize] != b'#' {
            i += dir;
        }

        if map[i as usize] == b'#' {
            return;
        }

        let a = *pos as usize;
        let b = i as usize;

        if dir > 0 {
            map.copy_within(a..b, a + 1);
            map[a] = b'.';
        } else {
            map.copy_within(b + 1..a + 1, b);
            map[a] = b'.';
        }

        *pos += dir;
        return;
    }

    // init with the robots position.
    try_push.push_front(pos.add(dir));

    
    // get the next position to try and check it.
    while let Some(pos) = try_push.pop_front() {
        let i = pos;
        // println!("i: {i}");
    
        match map[i as usize] {
            // hit a wall.
            // Can't advance so return with no change.
            b'#' => {
                // reset lists.
                try_push.clear();
                to_push.clear();
                return;
            },
            // empty space, nothing further to check here.
            b'.' => continue,
            // robots position.
            b'@' => {
                try_push.push_back(&pos + dir);
                to_push.push(pos);
            },
            c if c == b'[' || c == b']' => {
                // get the left and right components of the box.
                let (left, right);
                if c == b'[' {
                    right = pos + 1;
                    left = pos;
                } else {
                    left = pos - 1;
                    right = pos;
                }

                // check to see if this box has already been handled.
                if to_push.last().is_some_and(|p| *p == right) {
                    continue;
                }
    
                try_push.push_back(left + dir);
                try_push.push_back(right + dir);

                to_push.push(left);
                to_push.push(right);
            },
            _ => panic!("WHAT ARE YOU?"),
        }
    }
    
    // advance each element in reverse order.
    // from the empty space back to the robot.
    to_push.drain(..)
        .rev()
        .for_each(|p| {
            let i = p as usize;
            let ni = (p + dir) as usize;
            map[ni] = map[i];
            map[i] = b'.';
        });
    
    *pos += dir;
}

fn part_2(input: &str) -> String {
    let (map, moves) = input.trim().split_once("\n\n").unwrap();

    let lines = map.split("\n").collect::<Vec<_>>();
    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};

    let mut map = lines
        .iter()
        .flat_map(|line| {
            line.as_bytes()
        })
        .copied()
        .collect::<Vec<_>>();

    let mut fat_map = map.clone()
        .into_iter()
        .flat_map(|c| match c {
            b'#' => b"##",
            b'O' => b"[]",
            b'.' => b"..",
            b'@' => b"@.",
            _ => panic!(""),
        })
        .copied()
        .collect::<Vec<_>>();

    let fat_bounds = 2 * &bounds;

    let mut pos = Point::from_idx(
        map.iter().position(|c| *c == b'@').unwrap(),
        &bounds
    );

    let mut fat_pos = Point::from_idx(
        fat_map.iter().position(|c| *c == b'@').unwrap(),
        &fat_bounds
    );

    let moves = moves
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    for m in moves {
        // println!("{:?}", String::from_utf8(vec![m]));
        // print_grid(&map, bounds.x);
        // println!("\n");

        let dir_idx = match m {
            b'>' => 0,
            b'<' => 1,
            b'v' => 2,
            b'^' => 3,
            _ => panic!("ARGHHGHGHGHG"),
        };

        let dir = DIRS[dir_idx];
        let npos = &fat_pos + &dir;

        let ni = npos.to_idx(&fat_bounds);
        match fat_map[ni] {
            b'#' => {},
            b'.' => {
                fat_map[fat_pos.to_idx(&fat_bounds)] = b'.';
                fat_pos = npos;
                fat_map[ni] = b'@';
            }
            c if c == b'[' || c == b']' => {
                if push2(&mut fat_map, &npos, &dir, &fat_bounds, true) {
                    push2(&mut fat_map, &npos, &dir, &fat_bounds, false);
                    fat_map[fat_pos.to_idx(&fat_bounds)] = b'.';
                    fat_pos = npos;
                    fat_map[ni] = b'@';
                }
            }
            _ => panic!("WHAT ARE YOU! {:?}", String::from_utf8(fat_map[ni..ni+1].to_vec())),
        }

        let npos = &pos + &dir;

        let ni = npos.to_idx(&bounds);
        match map[ni] {
            b'#' => {},
            b'.' => {
                map[pos.to_idx(&bounds)] = b'.';
                pos = npos;
                map[ni] = b'@';
            }
            b'O' => {
                if push(&mut map, &npos, &dir, &bounds) {
                    map[pos.to_idx(&bounds)] = b'.';
                    pos = npos;
                    map[ni] = b'@';
                }
            }
            _ => panic!("WHAT ARE YOU! {:?}", String::from_utf8(map[ni..ni+1].to_vec())),
        }
    }

    // print_grid(&map, bounds.x);
    // println!("\n");

    let mut p2 = 0;
    for (i, c) in fat_map.iter().enumerate() {
        if *c == b'[' {
            let pos = Point::from_idx(i, &fat_bounds);
            let score = 100 * pos.y + pos.x;
            // println!("i: {i}, Pos: {:?}, score: {score}", pos);
            p2 += score;
        }
        
    }

    // print_grid(&map, bounds.x);
    // println!("\n");

    let mut p1 = 0;
    for (i, c) in map.iter().enumerate() {
        if *c == b'O' {
            let pos = Point::from_idx(i, &bounds);
            let score = 100 * pos.y + pos.x;
            // println!("i: {i}, Pos: {:?}, score: {score}", pos);
            p1 += score;
        }
        
    }

    format!("Part 1: {p1}\nPart 2: {p2}")
}

fn push2(map: &mut Vec<u8>, pos: &Point, dir: &Point, bounds: &Point, test: bool) -> bool {
    let i = pos.to_idx(bounds);

    match map[i] {
        b'#' => false,
        b'.' => true,
        c if c == b'[' || c == b']' => {
            if dir.x != 0 {
                let npos = pos + dir;
                if push2(map, &npos, dir, bounds, test) {
                    if !test {
                        let ni = npos.to_idx(bounds);
                        map[ni] = c;
                        map[i] = b'.';
                    }

                    true
                } else {
                    false
                }
            } else {
                let (left, right);
                if c == b'[' {
                    left = *pos;
                    right = pos + &DIRS[0];
                } else {
                    right = *pos;
                    left = pos + &DIRS[1];
                }

                let nl = &left + dir;
                let nr = &right + dir;

                if push2(map, &nl, dir, bounds, test) && push2(map, &nr, dir, bounds, test) {
                    if !test {
                        map[left.to_idx(bounds)] = b'.';
                        map[right.to_idx(bounds)] = b'.';

                        map[nl.to_idx(bounds)] = b'[';
                        map[nr.to_idx(bounds)] = b']';
                    }
                    true
                } else {
                    false
                }
            }
        }, 
        b'@' => panic!("SOMEHOW WE HIT THE ROBOT!"),
        _ => panic!("WHAT ARE YOU! {:?}", i),
    }
}

fn part_1(input: &str) -> String {
    let (map, moves) = input.trim().split_once("\n\n").unwrap();

    let lines = map.split("\n").collect::<Vec<_>>();
    let bounds = Point{x: lines[0].len() as i64, y: lines.len() as i64};

    let mut map = lines
        .iter()
        .flat_map(|line| {
            line.as_bytes()
        })
        .copied()
        .collect::<Vec<_>>();

    let mut robot_pos = Point::from_idx(
        map.iter().position(|c| *c == b'@').unwrap(),
        &bounds
    );

    let moves = moves
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    for m in moves {
        // print_grid(&map, bounds.x);
        // println!("\n");

        let dir_idx = match m {
            b'>' => 0,
            b'<' => 1,
            b'v' => 2,
            b'^' => 3,
            _ => panic!("ARGHHGHGHGHG"),
        };

        let dir = DIRS[dir_idx];
        let npos = &robot_pos + &dir;

        let ni = npos.to_idx(&bounds);
        match map[ni] {
            b'#' => continue,
            b'.' => {
                map[robot_pos.to_idx(&bounds)] = b'.';
                robot_pos = npos;
                map[ni] = b'@';
            }
            b'O' => {
                if push(&mut map, &npos, &dir, &bounds) {
                    map[robot_pos.to_idx(&bounds)] = b'.';
                    robot_pos = npos;
                    map[ni] = b'@';
                }
            }
            _ => panic!("WHAT ARE YOU! {:?}", String::from_utf8(map[ni..ni+1].to_vec())),
        }
    }

    // print_grid(&map, bounds.x);
    // println!("\n");

    let mut count = 0;
    for (i, c) in map.iter().enumerate() {
        if *c == b'O' {
            let pos = Point::from_idx(i, &bounds);
            let score = 100 * pos.y + pos.x;
            // println!("i: {i}, Pos: {:?}, score: {score}", pos);
            count += score;
        }
        
    }

    format!("Part 1: {count}")
}

fn push(map: &mut Vec<u8>, pos: &Point, dir: &Point, bounds: &Point) -> bool {
    let i = pos.to_idx(bounds);

    match map[i] {
        b'#' => false,
        b'.' => true,
        b'O' => {
            let npos = pos + dir;
            if push(map, &npos, dir, bounds) {
                map[i] = b'.';
                map[npos.to_idx(bounds)] = b'O';
                true
            } else {
                false
            }
        },
        b'@' => panic!("SOMEHOW WE HIT THE ROBOT!"),
        _ => panic!("WHAT ARE YOU! {:?}", i),
    }
}

fn print_grid(grid: &Vec<u8>, width: i64) {
    for line in grid
        .chunks(width.try_into().unwrap())
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>() {
        println!("{}", line);
    }
}
