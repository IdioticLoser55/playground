#![feature(try_find)]

use std::fs;
use std::time;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("My_Mess: \n{}\n", bench(&input, my_mess));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

#[derive(Clone, Debug)]
struct FileItem {
    position: usize,
    file_id: usize,
    size: usize,
}

#[derive(Clone, Debug)]
struct FreeItem {
    position: usize,
    size: usize,
}

fn my_mess(input: &str) -> String {
    let mut checksum = 0;

    let mut file_stack: Vec<FileItem> = Vec::new();
    let mut free_stack: Vec<FreeItem> = Vec::new();

    let mut current_id = 0;
    let mut current_position = 0;
    let chunks_iter = input.as_bytes().trim_ascii().chunks_exact(2);
    let remainder = chunks_iter.remainder();
    
    // let mut test: Vec<String> = Vec::new();

    for chunk in chunks_iter {
        let file_item = FileItem{
            position: current_position,
            file_id: current_id,
            size: parse_digit(chunk[0])
        };
        let free_item = FreeItem {
            position: file_item.position + file_item.size,
            size: parse_digit(chunk[1])
        };


        // Fib?
        checksum += (file_item.position..file_item.position + file_item.size).fold(0, |acc, pos| acc + pos * file_item.file_id);
        // (current_position..current_position + file_item.size).for_each(|_| test.push(file_item.file_id.to_string()));
        // (current_position+file_item.size..current_position + file_item.size + free_item.size).for_each(|_| test.push(".".to_string()));

        current_id += 1;
        current_position += file_item.size + free_item.size;


        file_stack.push(file_item);
        free_stack.push(free_item);
    }


    if !remainder.is_empty() {
        let file_item = FileItem {
            position: current_position,
            file_id: current_id,
            size: parse_digit(remainder[0]),
        };

        // Fib?
        checksum += (file_item.position..file_item.position + file_item.size).fold(0, |acc, pos| acc + pos * file_item.file_id);
        // (current_position..current_position + file_item.size).for_each(|_| test.push(file_item.file_id.to_string()));

        current_id += 1;
        current_position += file_item.size;

        file_stack.push(file_item);
    }

    // println!("ICheck: {}, FILES: {:?}", checksum, test);
    let mut p2_checksum = checksum;
    
    let mut free_stack_iter = free_stack.clone();
    let mut free_stack_iter = free_stack_iter.iter_mut();

    for file_item in file_stack.iter().rev() {
        // println!("File Item: {:?}", file_item);
        let Some(Some(free_item)) = free_stack
            .iter_mut()
            .try_find(|free_item| if free_item.position >= file_item.position {
                None
            } else {
                    Some(free_item.size >= file_item.size)
            })
        else {
            continue;
        };

        // println!("Free Item: {:?}", free_item);
        // println!("ICheck: {}, FILES: {:?}", p2_checksum, test);

        p2_checksum -= (file_item.position .. file_item.position + file_item.size)
            .fold(0, |acc, pos| acc + pos * file_item.file_id);

        // (file_item.position .. file_item.position + file_item.size)
        //     .for_each(|pos| test[pos] = ".".to_string());
        // println!("Files Removed: Checksum: {}, Files: {:?}", p2_checksum, test);

        p2_checksum += (free_item.position..free_item.position + file_item.size)
            .fold(0, |acc, pos| acc + pos * file_item.file_id);

        // (free_item.position..free_item.position + file_item.size)
        //     .for_each(|pos| test[pos] = file_item.file_id.to_string());
        // println!("Files Inserted: Checksum: {}, Files: {:?}", p2_checksum, test);

        free_item.position += file_item.size;
        free_item.size -= file_item.size;
    }

    // println!("HHMMM: {}", test.iter().enumerate().filter_map(|(i, s)| if s == "." { None } else {Some(i * s.parse::<usize>().unwrap())}).sum::<usize>());

    let mut file_stack_iter = file_stack.iter_mut().rev();
    let mut file_item: &mut FileItem = &mut FileItem{position: 0, file_id: 0, size: 0};
    let mut free_item: &mut FreeItem = &mut FreeItem{position: 0, size: 0};

    loop {
        // Need to keep track of current position. Or will start adding start files to end files.
        // println!("LOOP Start: free_item: {:?}, file_item: {:?}", free_item, file_item);

        if free_item.size == 0 {
            let Some(x) = free_stack_iter.next() else {break};
            free_item = x;
        }

        if file_item.size == 0 {
            let Some(x) = file_stack_iter.next() else {break};
            file_item = x;
        }

        // println!("NEW ITEMS: free_item: {:?}, file_item: {:?}", free_item, file_item);

        if free_item.position >= file_item.position {
            break;
        }

        let files_to_shift = free_item.size.min(file_item.size).min(file_item.position - free_item.position);
        // println!("shift: {}", files_to_shift);


        checksum -= (file_item.position + file_item.size - files_to_shift .. file_item.position + file_item.size)
            .fold(0, |acc, pos| acc + pos * file_item.file_id);
        // (file_item.position + file_item.size - files_to_shift .. file_item.position + file_item.size)
        //    .for_each(|pos| test[pos] = b'.');

        // println!("Files Removed: Checksum: {}, {:?}", checksum, String::from_utf8(test.to_vec()).unwrap());

        checksum += (free_item.position..free_item.position + files_to_shift)
            .fold(0, |acc, pos| acc + pos * file_item.file_id);
        // (free_item.position..free_item.position + files_to_shift)
        //    .for_each(|pos| test[pos] = b'0' + u8::try_from(file_item.file_id).unwrap());

        // println!("Files Inserted: Checksum: {}, {:?}", checksum, String::from_utf8(test.to_vec()).unwrap());

        free_item.size -= files_to_shift;
        free_item.position += files_to_shift;

        file_item.size -= files_to_shift;
    }

    format!("Part 1: {checksum}\nPart 2: {p2_checksum}")
}

fn parse_number(number: &[u8]) -> usize {
    number.iter()
        .fold(0, |acc, digit| {
            acc * 10 + parse_digit(*digit)
        })
}

fn parse_digit(number: u8) -> usize {
    usize::from(number) - usize::from(b'0')
}
