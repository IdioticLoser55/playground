use std::fs;
use std::time;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("MyMess {}\n", bench(&input, my_mess));
    println!("MyMessTidied {}\n", bench(&input, my_mess_tidied));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}


fn get_dir_val(dx: i32, dy: i32) -> u8 {
    if dx == 1 {
        1
    } else if dx == -1 {
        2
    } else if dy == 1 {
        4
    } else {
        8
    }
}

fn walk_grid(grid: &Vec<u8>, width: i32, height: i32, sx: i32, sy: i32, dx: i32, dy: i32, seen: &mut Vec<u8>) -> bool {
    let (mut pos, mut apos): (i32, i32);
    let (mut px, mut py) = (sx, sy);
    let (mut ax, mut ay);
    let (mut dx, mut dy) = (dx, dy);

    pos = sy * width + sx;
    loop {
        (ax, ay) = (px + dx, py + dy);
        if ax >= width || ax < 0 || ay < 0 || ay >= height {
            break;
        }

        apos = ay * width + ax;
        if grid[usize::try_from(apos).unwrap()] == b'#' {
            (dx, dy) = (-dy, dx);
        } else {
            (px, py) = (ax, ay);
            pos = apos;
        }

        let val: &mut u8 = seen.get_mut::<usize>(pos.try_into().unwrap()).unwrap();
        let dir_val = get_dir_val(dx, dy);

        if *val & dir_val > 0 {
            return false;
        } else {
            *val |= dir_val;
        }

    }

    true
}

fn my_mess_tidied(input: &str) -> String {
    // for line in grid
    //     .chunks(width.try_into().unwrap())
    //     .map(|line| String::from_utf8(line.to_vec()).unwrap())
    //     .collect::<Vec<String>>() {
    //     println!("{}", line);
    // }
    
    let input = input.trim();

    // Split text on line breaks.
    let lines = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .collect::<Vec<&[u8]>>();

    // Get width and height of grid.
    let width: i32 = lines[0].len().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();

    // push grid into vec.
    let mut grid: Vec<u8> = Vec::with_capacity(usize::try_from(width * height).unwrap());
    lines.iter().for_each(|line| grid.extend_from_slice(line));

    // get start position.
    let start_position: i32 = grid
        .iter()
        .position(|&c| c == b'^')
        .unwrap()
        .try_into()
        .unwrap();

    // start x and start y.
    let (mut px, mut py): (i32, i32) = (start_position % width, start_position / width);
    let (mut dx, mut dy): (i32, i32) = (0, -1);
    let (mut ax, mut ay): (i32, i32);
    let mut pos: i32 = py * width + px;
    let mut apos: i32;

    // Used to keep track of visited squares.
    // Uses a bitwise flag to record the different directions visited from.
    // see get_dir_val
    let mut seen = vec![0; grid.len()];

    let mut looping_count = 0;
    loop {
        // update seen with current position and direction.
        seen[usize::try_from(pos).unwrap()] |= get_dir_val(dx, dy);

        // calculate advance position.
        (ax, ay) = (px + dx, py + dy);

        // check to see if out of bounds.
        if ax >= width || ax < 0 || ay < 0 || ay >= height {
            break;
        }

        apos = ay * width + ax;
        // Check to see if advance pos has been visited yet.
        if seen[usize::try_from(apos).unwrap()] == 0 {
            // not been visited yet.
            // So we'll try a new obstacle here.
            // And check to see if it loops.

            // record current symbol.
            let old = grid[usize::try_from(apos).unwrap()];
            // insert obstacle.
            grid[usize::try_from(apos).unwrap()] = b'#';

            // walk grid from here.
            if !walk_grid(&grid, width, height, px, py, dx, dy, &mut seen.clone()) {
                looping_count += 1;
            }

            // reset grid.
            grid[usize::try_from(apos).unwrap()] = old;
        }

        // check for obstacle ahead.
        if grid[usize::try_from(apos).unwrap()] == b'#' {
            // turn.
            (dx, dy) = (-dy, dx);
        } else {
            // move into space.
            (px, py) = (ax, ay);
            pos = apos;
        }
    }

    let count: usize = seen.iter().filter(|el| **el > 0).count();
    format!("Count: {count}\nLoop: {looping_count}")
}


fn my_mess(input: &str) -> String {
    // for line in grid
    //     .chunks(width.try_into().unwrap())
    //     .map(|line| String::from_utf8(line.to_vec()).unwrap())
    //     .collect::<Vec<String>>() {
    //     println!("{}", line);
    // }
    
    let input = input.trim();

    let lines = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .collect::<Vec<&[u8]>>();

    let width: i32 = lines[0].len().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();
    // println!("Width: {width}, Height: {height}");

    let mut grid: Vec<u8> = Vec::with_capacity(usize::try_from(width * height).unwrap());
    lines.iter().for_each(|line| grid.extend_from_slice(line));

    let position: i32 = grid
        .iter()
        .position(|&c| c == b'^')
        .unwrap()
        .try_into()
        .unwrap();
    let (ix, iy): (i32, i32) = (position % width, position / width);
    let (mut px, mut py): (i32, i32) = (ix, iy);

    let mut pos: i32;
    let (mut dx, mut dy): (i32, i32) = (0, -1);
    let (mut ax, mut ay): (i32, i32);

    loop {
        pos = py * width + px;
        grid[usize::try_from(pos).unwrap()] = b'x';

        // println!("X: {px}, Y: {py}; dx: {dx}, dy: {dy}");
        (ax, ay) = (px + dx, py + dy);

        if ax >= width || ax < 0 || ay < 0 || ay >= height {
            break;
        }

        pos = ay * width + ax;

        if grid[usize::try_from(pos).unwrap()] == b'#' {
            (dx, dy) = (-dy, dx);
            continue;
        }

        (px, py) = (ax, ay);
    }
    let visited = grid.iter().filter(|&&c| c == b'x').count();

    let possible_positions = grid
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if *val == b'x' && i != usize::try_from(position).unwrap() {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let mut looping_count = 0;
    let mut path_grid = grid.clone();
    for p in possible_positions {
        grid[p] = b'#';

        path_grid.fill(0);

        (px, py) = (ix, iy);
        (dx, dy) = (0, -1);

        loop {
            pos = py * width + px;

            let val: &mut u8 = path_grid.get_mut::<usize>(pos.try_into().unwrap()).unwrap();
            let dir_val = get_dir_val(dx, dy);

            if *val & dir_val > 0 {
                looping_count += 1;
                break;
            } else {
                *val = *val | dir_val;
            }

            // println!("X: {px}, Y: {py}; dx: {dx}, dy: {dy}");
            (ax, ay) = (px + dx, py + dy);

            if ax >= width || ax < 0 || ay < 0 || ay >= height {
                break;
            }

            pos = ay * width + ax;

            if grid[usize::try_from(pos).unwrap()] == b'#' {
                (dx, dy) = (-dy, dx);
                continue;
            }

            (px, py) = (ax, ay);
        }

        grid[p] = b'.';
    }

    format!("Part 1: {visited}\nPart 2: {looping_count}")
}
