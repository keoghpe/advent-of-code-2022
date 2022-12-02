use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn main() {
    let mut score = 0;

    if let Ok(lines) = read_lines("./src/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let mut split = l.split_whitespace();
                let a = split.next().unwrap();
                let b = split.next().unwrap();

                let their_move = match a {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => panic!("shit"),
                };

                let my_move = match b {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    "Z" => Move::Scissors,
                    _ => panic!("shit"),
                };

                let new_score = calculate_score(&their_move, &my_move);

                score += new_score;

                println!("{} {} {} {}", a, b, new_score, score);
            }
        }

        println!("{}", score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_score(their_move: &Move, my_move: &Move) -> i32 {
    let base_score = match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome_score = match calculate_outcome(&their_move, &my_move) {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    };

    base_score + outcome_score
}

fn calculate_outcome(their_move: &Move, my_move: &Move) -> Outcome {
    use Move::*;

    match [my_move, their_move] {
        [Rock, Rock] => Outcome::Draw,
        [Paper, Paper] => Outcome::Draw,
        [Scissors, Scissors] => Outcome::Draw,
        [Rock, Paper] => Outcome::Lose,
        [Rock, Scissors] => Outcome::Win,
        [Paper, Rock] => Outcome::Win,
        [Paper, Scissors] => Outcome::Lose,
        [Scissors, Rock] => Outcome::Lose,
        [Scissors, Paper] => Outcome::Win,
    }
}
