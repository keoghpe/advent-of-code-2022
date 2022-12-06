use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

static CONSECUTIVE_CHARS: usize = 4;

fn main() {
    let mut current_chars: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines("./src/day6.txt") {
        for line in lines {
            let l = line.unwrap();

            for (i, c) in l.chars().enumerate() {
                println!("{}", c);

                current_chars.push(c);

                if current_chars.len() > CONSECUTIVE_CHARS {
                    current_chars.remove(0);
                }

                if current_chars.len() == CONSECUTIVE_CHARS && all_unique(&current_chars) {
                    println!("{}", i + 1);
                    return;
                }
            }
        }
    }
}

fn all_unique(char_list: &Vec<char>) -> bool {
    let mut list_copy = char_list.clone();
    list_copy.sort();
    list_copy.dedup();
    list_copy.len() == CONSECUTIVE_CHARS
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
