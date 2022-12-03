use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/day3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                score += score_line(&l);
            }
        }
    }
    println!("{}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn score_line(line: &str) -> i32 {
    let first_half = &line[..(line.len() / 2)];
    let second_half = &line[(line.len() / 2)..];

    for c in first_half.chars() {
        if second_half.contains(c) {
            return letter_to_prio(&c);
        }
    }
    0
}

fn letter_to_prio(letter: &char) -> i32 {
    // a = 97
    // z = 122
    // A = 65
    // Z = 90

    let score = *letter as i32;
    if (97..123).contains(&score) {
        score - 96
    } else {
        score - 38
    }
}
