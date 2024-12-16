use std::fs;
use std::time;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
struct RouteHead {
    position: isize,
    direction: usize,
    score: usize,
}

// makes it so route head struct is sorted by the score.
impl Ord for RouteHead {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for RouteHead {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("FILE NOT FOUND");

    println!("my_attempt: \n{}\n", bench(&input, my_attempt));
}

fn bench(input: &str, f: fn(&str) -> String) -> String {
    let t0 = time::Instant::now();
    let ret = f(&input);
    println!("time used {:?}", t0.elapsed());

    ret
}

fn my_attempt(input: &str) -> String {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let width = lines[0].len() as isize;
    let height = lines.len() as isize;

    let dirs = vec![
        -1,
        1,
        -width,
        width,
    ];

    let mut grid: Vec<u8> = Vec::with_capacity((width * height) as usize);
    lines.iter()
        .for_each(|line| grid.extend_from_slice(line));
    let mut visited = vec![vec![usize::MAX; dirs.len()]; grid.len()];

    let start_pos = grid.iter().position(|c| *c == b'S').unwrap() as isize;
    let end_pos = grid.iter().position(|c| *c == b'E').unwrap();

    let mut points_to_check: BinaryHeap<Reverse<RouteHead>> = BinaryHeap::new();

    // init with starting pos.
    points_to_check.push(Reverse(RouteHead{
        position: start_pos,
        direction: 1,
        score: 0,
    }));

    // get point with LOWEST SCORE and loop while points remaining..
    while let Some(Reverse(route_head)) = points_to_check.pop() {
        // Check for a wall.
        if grid[route_head.position as usize] == b'#' {
            continue;
        }

        // check to see if already found a shorter route here. keeping track of the directions as
        // well.
        if route_head.score >= visited[route_head.position as usize][route_head.direction] {
            continue
        }
        // mark as visited and record distance.
        visited[route_head.position as usize][route_head.direction] = route_head.score;

        // break if reached end.
        // Because of min heap this will be shortest route.
        if route_head.position == end_pos as isize {
            break;
        }

        // push next points to heap.
        match route_head.direction {
            0 | 1 => {
                // horizontal direction. So push an advancement in the same direction.
                // and add rotations up and down as well.
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position + dirs[route_head.direction],
                    direction: route_head.direction,
                    score: route_head.score + 1
                }));
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position,
                    direction: 2,
                    score: route_head.score + 1000
                }));
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position,
                    direction: 3,
                    score: route_head.score + 1000
                }));
            },
            2 | 3=> {
                // vertical direction. So push an advancement in the same direction.
                // and add rotations left and right as well.
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position + dirs[route_head.direction],
                    direction: route_head.direction,
                    score: route_head.score + 1
                }));
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position,
                    direction: 0,
                    score: route_head.score + 1000
                }));
                points_to_check.push(Reverse(RouteHead{
                    position: route_head.position,
                    direction: 1,
                    score: route_head.score + 1000
                }));
            },
            _ => unreachable!()
        }
    }

    // get lowest score at finishing position.
    let score = visited[end_pos].iter().min().unwrap();
    // and matching direction.
    let dir = visited[end_pos].iter().position(|s| s == score).unwrap();

    let mut seats = vec![false; grid.len()];

    // init with end point. This time as a max heap.
    let mut points_to_check: BinaryHeap<RouteHead> = BinaryHeap::new();
    points_to_check.push(RouteHead{
        position: end_pos as isize,
        direction: dir,
        score: *score
    });

    // get HIGHEST SCORE and loop while points remain.
    while let Some(route_head) = points_to_check.pop() {
        // Check for wall.
        if grid[route_head.position as usize] == b'#' {
            continue;
        }

        // We're back-tracing the fastest routes, so only want squares where the score and
        // directions match.
        if route_head.score != visited[route_head.position as usize][route_head.direction] {
            continue;
        }

        // mark this square as being along the fastest route.
        seats[route_head.position as usize] = true; 

        // given a direction do the inverse of above.
        match route_head.direction {
            0 | 1 => {
                points_to_check.push(RouteHead{
                    position: route_head.position - dirs[route_head.direction],
                    direction: route_head.direction,
                    score: route_head.score - 1
                });
                points_to_check.push(RouteHead{
                    position: route_head.position,
                    direction: 2,
                    score: route_head.score - 1000,
                });
                points_to_check.push(RouteHead{
                    position: route_head.position,
                    direction: 3,
                    score: route_head.score - 1000,
                });
            },
            2 | 3=> {
                points_to_check.push(RouteHead{
                    position: route_head.position - dirs[route_head.direction],
                    direction: route_head.direction,
                    score: route_head.score - 1
                });
                points_to_check.push(RouteHead{
                    position: route_head.position,
                    direction: 0,
                    score: route_head.score - 1000,
                });
                points_to_check.push(RouteHead{
                    position: route_head.position,
                    direction: 1,
                    score: route_head.score - 1000,
                });
            },
            _ => unreachable!()
        }
    }

    // count all the seats marked.
    let count = seats.iter().filter(|v| if **v {true} else {false}).count();

    format!("Part 1: {score}\nPart 2: {count}")
}


fn print_grid(grid: &Vec<u8>, width: usize) {
    for line in grid
        .chunks(width)
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>() {
        println!("{}", line);
    }
}
