use std::str::Split;
use std::time::Instant;
use advent_lib;

fn main() {
    let now = Instant::now();

    let text = advent_lib::read_file(&String::from("day2/resources/input.txt"));
    let plays: Split<&str> = text.split("\n");

    let mut opponent_move: i32;
    //let mut my_move: i32;
    let mut score = 0;
    let mut moves: Split<&str>;
    
    for play in plays{
        moves = play.split(" ");
        
        opponent_move = match moves.next().unwrap() {
            "A" => 1,
            "B" => 2,
            _ => 3,
        };
        
        score += match moves.next().unwrap() {
            "X" => if opponent_move == 1 { 3 } else { opponent_move - 1 },
            "Y" => opponent_move + 3,
            _ => (if opponent_move == 3 { 1 } else { opponent_move + 1 }) + 6,
        };
        
        //score += my_move;
        //
        //score += match my_move - opponent_move {
        //    1 => 6,
        //    0 => 3,
        //    -2 => 6,
        //    _ => 0,
        //};
    }
    
    let elapsed = now.elapsed();
    println!("Score: {:}\nElapsed: {:.2?}", score, elapsed);
    
}
