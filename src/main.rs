use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut current_total = 0;
    let mut top_totals: [i32; 3] = [0, 0, 0];

    if let Ok(lines) = read_lines("./src/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                match l.parse::<i32>() {
                    Ok(number) => {
                        current_total += number;
                    }
                    Err(_) => {
                        if current_total > top_totals[0] {
                            top_totals[0] = current_total;
                            top_totals.sort();
                        }
                        current_total = 0;
                    }
                }
            }
        }
    }

    println!("{}", top_totals.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
