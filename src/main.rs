use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/day4.txt") {
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
            let mut split_line = l.split(",");
            // let elf_a = split_line.next();
            // let elf_b = split_line.next();

            let mut elf_a_coords = split_line
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<i32>().unwrap());

            let mut elf_b_coords = split_line
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse::<i32>().unwrap());

            let elf_a_start = elf_a_coords.next().unwrap();
            let elf_a_end = elf_a_coords.next().unwrap();

            let elf_b_start = elf_b_coords.next().unwrap();
            let elf_b_end = elf_b_coords.next().unwrap();

            println!("a start: {} a end: {}", elf_a_start, elf_a_end);
            println!("b start: {} b end: {}", elf_b_start, elf_b_end);

            if enclose(elf_a_start, elf_a_end, elf_b_start, elf_b_end)
                || enclose(elf_b_start, elf_b_end, elf_a_start, elf_a_end)
            {
                println!("enclosed!");
                score += 1;
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

fn enclose(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    a_start <= b_start && a_end >= b_end
}
