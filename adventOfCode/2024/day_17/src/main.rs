use std::fs;
use std::time;
use std::collections::VecDeque;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("my_attempt: \n{}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}


fn my_attempt(input: &str) -> String {
    let lines = input.trim().lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut registers = lines[..3]
        .iter()
        .map(|line| parse_register(line))
        .collect::<Vec<_>>();


    println!("{:?}", registers);
    let program = lines[4]
        .split(|c| *c == b' ')
        .nth(1)
        .unwrap()
        .split(|c| *c == b',')
        .map(|c| parse_digit(c[0]))
        .collect::<Vec<_>>();

    println!("{:?}", program);
    
    let mut output: Vec<usize> = Vec::new();
    evaluate_program(&program, &registers, &mut output);

    let p1 = output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    let mut final_vals = Vec::new();
    let mut possible_a = VecDeque::new();
    possible_a.push_back((0, 0));

    'outer: while let Some((a, i)) = possible_a.pop_front() {
        let a = a << 3;

        for aa in 0..=7 {
            registers[0] = a + aa;
            output.clear();

            let r = evaluate_program_to_first_out(&program, &registers);

            if r.is_some_and(|r| program[program.len() - 1 - i] == r) {
                if i == program.len() - 1 {
                    final_vals.push(a + aa);
                } else {
                    possible_a.push_back((a + aa, i + 1));
                }
            }
        }

        for v in final_vals.iter() {
            if possible_a.iter().all(|(a, _i)| v < a) {
                break 'outer;
            }
        }
    }


    let p2 = final_vals.iter().min().unwrap();

    format!("Part 1: {p1}\nPart 2: {p2}")
}


fn evaluate_program_to_first_out(program: &Vec<usize>, registers: &Vec<usize>) -> Option<usize> {
    let mut registers = registers.clone();

    let mut pointer = 0;
    while pointer < program.len() {
        let (opcode, operand) = (program[pointer], program[pointer + 1]);
        match opcode {
            0 => registers[0] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            1 => registers[1] = registers[1] ^ operand,
            2 => registers[1] = to_combo(&registers, operand) % 8,
            3 => if registers[0] != 0 {pointer = operand; continue;}
            4 => registers[1] = registers[1] ^ registers[2],
            5 => return Some(to_combo(&registers, operand) % 8),
            6 => registers[1] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            7 => registers[2] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            _ => unreachable!(),
        }

        pointer += 2;
    }

    None
}

fn evaluate_program(program: &Vec<usize>, registers: &Vec<usize>, output: &mut Vec<usize>) {
    let mut registers = registers.clone();

    let mut pointer = 0;
    while pointer < program.len() {
        let (opcode, operand) = (program[pointer], program[pointer + 1]);
        match opcode {
            0 => registers[0] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            1 => registers[1] = registers[1] ^ operand,
            2 => registers[1] = to_combo(&registers, operand) % 8,
            3 => if registers[0] != 0 {pointer = operand; continue;}
            4 => registers[1] = registers[1] ^ registers[2],
            5 => output.push(to_combo(&registers, operand) % 8),
            6 => registers[1] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            7 => registers[2] = registers[0] / 2_usize.pow(to_combo(&registers, operand) as u32),
            _ => unreachable!(),
        }

        pointer += 2;
    }
}

fn to_combo(registers: &Vec<usize>, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4..=6 => registers[operand - 4],
        _ => unreachable!(),
    }
}

fn parse_register(line: &[u8]) -> usize {
    line
        .split(|c| *c == b' ')
        .nth(2)
        .unwrap()
        .into_iter()
        .fold(0, |acc, c| acc * 10 + parse_digit(*c))
}

fn parse_digit(number: u8) -> usize {
    usize::from(number - b'0')
}
