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

    let mut slices_to_search:Vec<Box<dyn Iterator<Item = usize>>> = Vec::with_capacity(2 * (width + height));

    // horizontal slices.
    for y in (0..height*width).step_by(width) {
        slices_to_search.push(Box::new(y..y+width));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // vertical slices.
    for x in 0..width {
        slices_to_search.push(Box::new((x..height*width).step_by(width)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - top half
    for x in 0..width {
        slices_to_search.push(Box::new((x..1+(x*width)).step_by(width - 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - bottom half
    for y in (width * 2 - 1..width*height).step_by(width) {
        slices_to_search.push(Box::new((y..width*height).step_by(width - 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    // diagonal to the right slices - top half
    for x in 0..width {
        slices_to_search.push(Box::new((x..((width - x)*width)).step_by(width + 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    for (i, y) in (width..width*height).step_by(width).enumerate() {
        slices_to_search.push(Box::new((y..y+(width - i - 1) * width).step_by(width + 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    let mut matches = 0;

    // Loops through of the slices created.
    for slice in slices_to_search.iter_mut() {
        let selection: Vec<u8> = slice.map(|idx| letters[idx]).collect();
        matches += selection.windows(pattern_size).filter(|&w| w == pattern || w == rpattern).count();
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

    let mut slices_to_search:Vec<Box<dyn Iterator<Item = usize>>> = Vec::with_capacity(2 * (width + height));

    // diagonal to the left slices - top half
    for x in 0..width {
        slices_to_search.push(Box::new((x..1+(x*width)).step_by(width - 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    // diagonal to the left slices - bottom half
    for y in (width * 2 - 1..width*height).step_by(width) {
        slices_to_search.push(Box::new((y..width*height).step_by(width - 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    // diagonal to the right slices - top half
    for x in 0..width {
        slices_to_search.push(Box::new((x..((width - x)*width)).step_by(width + 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }

    for (i, y) in (width..width*height).step_by(width).enumerate() {
        slices_to_search.push(Box::new((y..y+(width - i - 1) * width).step_by(width + 1)));
        // println!("{:#?}", slices_to_search.last().unwrap());
    }


    for slice in slices_to_search.iter_mut() {
        let indices: Vec<usize> = slice.collect();
        if indices.len() < pattern_size {
            continue;
        }

        let selection: Vec<u8> = indices.iter().map(|&idx| letters[idx]).collect();
        selection.windows(pattern_size)
            .enumerate()
            .filter_map(|(i, w)| {
                if w == pattern || w == rpattern {
                    Some(i)
                } else {
                    None
                }
            })
            .for_each(|i| {
                pattern_matches[indices[i + pattern_size / 2]] += 1;
            });
    }

    // Once found all matches looks for all places with more than 1 match. = x.
    // and counts those.
    let x_matches = pattern_matches
        .into_iter()
        .filter(|&match_count| match_count > 1)
        .count();
    format!("X-Matches: {x_matches}")
}
