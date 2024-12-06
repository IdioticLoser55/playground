use std::fs;
use std::time;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("MyMess {}\n", bench(&input, my_mess));
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


fn my_mess(input: &str) -> String {
    let input = input.trim();

    let lines = input.as_bytes().split(|&c| c == b'\n').collect::<Vec<&[u8]>>();

    let width: i32 = lines[0].len().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();
    // println!("Width: {width}, Height: {height}");

    let mut grid: Vec<u8> = Vec::with_capacity(usize::try_from(width * height).unwrap());
    lines.iter().for_each(|line| grid.extend_from_slice(line));

    let position: i32 = grid.iter().position(|&c| c == b'^').unwrap().try_into().unwrap();
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

    let mut path = grid.clone();
    path.fill(0);

    let possible_positions = grid
        .iter()
        .enumerate()
        .filter_map(|(i, val)| 
            if *val == b'x' && i != usize::try_from(position).unwrap() {
                Some(i)
            } else {
                None
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

