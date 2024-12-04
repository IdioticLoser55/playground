use std::fs;
use std::time;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}\n", bench(&input, part1));
    println!("Part 2: {}\n", bench(&input, part2));
}


fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn part1 (input: &str) -> String {
    let bytes = input.as_bytes();

    let letters: Vec<u8> = bytes.iter().filter_map(|&c| if c.is_ascii_whitespace() {None} else {Some(c.clone())}).collect();

    let width = bytes.iter().position(|&c| c.is_ascii_whitespace()).unwrap();
    let height = letters.len() / width;

    let pattern = b"XMAS";
    let rpattern: &[u8] = &pattern.iter().rev().map(|&c| c.clone()).collect::<Vec<u8>>();
    let pattern_size = pattern.len();

    let mut slices_to_search:Vec<Vec<usize>> = Vec::new();

    // horizontal slices.
    for y in (0..height*width).step_by(width) {
        slices_to_search.push((y..y+width).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // vertical slices.
    for x in 0..width {
        slices_to_search.push((x..height*width).step_by(width).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - top half
    for x in 0..width {
        slices_to_search.push((x..1+(x*width)).step_by(width - 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - bottom half
    for y in (width * 2 - 1..width*height).step_by(width) {
        slices_to_search.push((y..width*height).step_by(width - 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    // diagonal to the right slices - top half
    for x in 0..width {
        slices_to_search.push((x..((width - x)*width)).step_by(width + 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    for (i, y) in (width..width*height).step_by(width).enumerate() {
        slices_to_search.push((y..y+(width - i - 1) * width).step_by(width + 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    let mut matches = 0;
    let window: &mut [u8] = &mut vec![0; pattern_size];

    // Loops through of the slices created.
    for slice in slices_to_search.iter() {
        // Checks if the slice is big enough.
        if slice.len() < pattern_size {
            continue;
        }

        // constructs a sliding window the size of the pattern across the slice.
        for window_indices in slice.windows(pattern_size) {

            // extracts the desired elements into window.
            window_indices.iter().enumerate().for_each(|(enumeration, &idx)| window[enumeration] = letters[idx]);

            // checks to see if there is a match.
            if window == pattern || window == rpattern {
                matches += 1;
            }
        }
    }

    format!("Matches: {matches}")
}

fn part2(input: &str) -> String {
    let bytes = input.as_bytes();

    let letters: Vec<u8> = bytes.iter().filter_map(|&c| if c.is_ascii_whitespace() {None} else {Some(c.clone())}).collect();

    let width = bytes.iter().position(|&c| c.is_ascii_whitespace()).unwrap();
    let height = letters.len() / width;

    let pattern = b"MAS";
    let rpattern: &[u8] = &pattern.iter().rev().map(|&c| c.clone()).collect::<Vec<u8>>();
    let pattern_size = pattern.len();
    let mut pattern_matches: Vec<usize> = vec![0; letters.len()];

    let mut slices_to_search:Vec<Vec<usize>> = Vec::new();

    // // horizontal slices.
    // for y in (0..height*width).step_by(width) {
    //     slices_to_search.push((y..y+width).collect::<Vec<usize>>());
    //     // println!("{:#?}", slices_to_search.last().unwrap());
    // }

    // // vertical slices.
    // for x in 0..width {
    //     slices_to_search.push((x..height*width).step_by(width).collect::<Vec<usize>>());
    //     // println!("{:#?}", slices_to_search.last().unwrap());
    // }

    // diagonal to the left slices - top half
    for x in 0..width {
        slices_to_search.push((x..1+(x*width)).step_by(width - 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - bottom half
    for y in (width * 2 - 1..width*height).step_by(width) {
        slices_to_search.push((y..width*height).step_by(width - 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    // diagonal to the right slices - top half
    for x in 0..width {
        slices_to_search.push((x..((width - x)*width)).step_by(width + 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    for (i, y) in (width..width*height).step_by(width).enumerate() {
        slices_to_search.push((y..y+(width - i - 1) * width).step_by(width + 1).collect::<Vec<usize>>());
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    let window: &mut [u8] = &mut vec![0; pattern_size];

    for slice in slices_to_search.iter() {
        if slice.len() < pattern_size {
            continue;
        }

        for window_indices in slice.windows(pattern_size) {
            window_indices.iter().enumerate().for_each(|(enumeration, &idx)| window[enumeration] = letters[idx]);

            // same as part 1. But counts matches per location.
            if window == pattern || window == rpattern {
                pattern_matches[window_indices[window_indices.len() / 2]] += 1;
            }
        }
    }

    // Once found all matches looks for all places with more than 1 match. = x.
    // and counts those.
    let x_matches = pattern_matches
        .into_iter()
        .filter(|&match_count| match_count > 1)
        .count();
    format!("X-Matches: {x_matches}")
}
