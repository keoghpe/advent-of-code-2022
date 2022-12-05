use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut score = 0;
    let mut grid_parsing_complete = false;
    let mut stacks: [Vec<char>; 9] = Default::default();

    if let Ok(lines) = read_lines("./src/day5.txt") {
        for line in lines {
            //     if let Ok(l) = line {
            //         for c in l.chars() {
            //             if second_line.unwrap().contains(&c.to_string())
            //                 && third_line.unwrap().contains(&c.to_string())
            //             {
            //                 score += letter_to_prio(&c);
            //             }
            //         }
            let l = line.unwrap();
            if grid_parsing_complete == false {
                println!("{}", l);
                let mut space_count = 0;

                for (i, c) in l.chars().enumerate() {
                    println!("{}", c);

                    match c {
                        'A'..='Z' => {
                            let index = i / 4;
                            stacks[index].insert(0, c);
                        }
                        _ => (),
                    }
                }
                // 1
                // 5
                // 9
                // 13
                // 17
                // [B] [N] [N] [N] [Q] [W] [L] [Q] [S]
                if l == "" {
                    grid_parsing_complete = true;
                }
            } else {
                let mut split_line = l.split(" ");
                split_line.next();

                let amount = split_line.next().unwrap().parse::<i32>().unwrap();
                split_line.next();
                let from = split_line.next().unwrap().parse::<i32>().unwrap();
                split_line.next();
                let to = split_line.next().unwrap().parse::<i32>().unwrap();

                for _ in 0..amount {
                    let from_val = stacks[(from - 1) as usize].pop();

                    match from_val {
                        Some(f_val) => stacks[(to - 1) as usize].push(f_val),
                        None => (),
                    }
                }
            }
        }
    }
    println!("{:?}", stacks);
    println!("{}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// [['B', 'Q', 'L', 'J', 'S', 'Z', 'R', 'B', 'N', 'Q', 'Z', 'G', 'Q']
// , ['F', 'N']
// , ['N']
// , ['D', 'T']
// , ['W', 'C', 'H', 'R', 'N', 'G']
// , ['L', 'R', 'T']
// , ['J', 'C', 'F', 'T', 'H', 'F', 'D', 'N', 'Z', 'C', 'F', 'P']
// , ['G', 'G', 'T', 'Q', 'N', 'J', 'J', 'R', 'M', 'Q', 'V', 'R', 'M', 'F']
// , ['S', 'S', 'N']]
