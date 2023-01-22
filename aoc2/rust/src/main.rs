
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors.
// The score for a single round is the score for the shape you selected 
// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round 
// (0 if you lost, 3 if the round was a draw, and 6 if you won).

#[derive(Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Sissors = 3,
}

#[derive(Debug)]
enum Result {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

fn main() {
    let file = File::open("guide.txt").unwrap();
    let reader = BufReader::new(file);
    let mut my_score = 0u64;
    for line in reader.lines() {
        if let Ok(round_line) = line {
            //println!("{round}");
            let round = round_line.split(" ").collect::<Vec<&str>>();
            if round.len() != 2 {
                panic!("round results length is not 2!");
            }
            let my_choice = match round[0] {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Sissors,
                _ => panic!("Error: Invalid choice {}", round[0]),
            };
            let opponent_choice = match round[1] {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Sissors,
                _ => panic!("Error: Invalid choice {}", round[1]),
            };
            //print!("My choice: {my_choice:#?}          Opponent Choice: {opponent_choice:#?}     ");
            let round_result = match (&my_choice, &opponent_choice) {
                (Choice::Rock, Choice::Rock) => Result::Draw,
                (Choice::Rock, Choice::Paper) => Result::Lost,
                (Choice::Rock, Choice::Sissors) => Result::Win,
                (Choice::Paper, Choice::Rock) => Result::Win,
                (Choice::Paper, Choice::Paper) => Result::Draw,
                (Choice::Paper, Choice::Sissors) => Result::Lost,
                (Choice::Sissors, Choice::Rock) => Result::Lost,
                (Choice::Sissors, Choice::Paper) => Result::Win,
                (Choice::Sissors, Choice::Sissors) => Result::Draw,
            };
            my_score += round_result as u64;
            my_score += my_choice as u64;
            //println!("Round result: {round_result:#?}");
        }
    }
    println!("Total Score: {my_score}");
}
