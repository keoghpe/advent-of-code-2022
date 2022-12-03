use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut score = 0;
    if let Ok(mut lines) = read_lines("./src/day3.txt") {
        // Consumes the iterator, returns an (Optional) String
        loop {
            let first_line;
            match lines.next() {
                Some(x) => first_line = x,
                None => break,
            }
            let second_line = lines.next().unwrap().unwrap();
            let third_line = lines.next().unwrap().unwrap();

            println!("first line: {:?}", first_line);
            println!("second_line line: {}", second_line);
            println!("third line: {}", third_line);

            for c in first_line.unwrap().chars() {
                if second_line.contains(&c.to_string()) && third_line.contains(&c.to_string()) {
                    println!("char: {}", c);
                    println!("prio for letter: {}", letter_to_prio(&c));
                    score += letter_to_prio(&c);
                    println!("score: {}", score);
                    break;
                }
            }
        }
        // for line in lines {
        //     if let Ok(l) = line {
        //         for c in l.chars() {
        //             if second_line.unwrap().contains(&c.to_string())
        //                 && third_line.unwrap().contains(&c.to_string())
        //             {
        //                 score += letter_to_prio(&c);
        //             }
        //         }
        //     }
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
