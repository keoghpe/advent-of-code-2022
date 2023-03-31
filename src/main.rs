use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// struct Node<'a> {
//     content: HashMap,
//     parent: &'a Node,
// }

// Nodes point to their children and their parent
// We need to have a pointer to the root node always
// We need to have a pointer to the current node

// cd / changes the current node pointer to the root node
// cd .. changes to the current node parent
// ls lists the current directories. It should add any that aren't children of the current node.

fn main() {
    // let root = Node::new();

    if let Ok(lines) = read_lines("./src/day7.txt") {
        for line in lines {
            let l = line.unwrap();
            println!("{}", l);

            match l.as_bytes()[0] as char {
                '$' => match &l[..4] {
                    "$ cd" => match l.split(' ').nth(2).unwrap() {
                        "/" => {
                            println!("change to root");
                        }
                        ".." => {
                            println!("naviate out");
                        }
                        dir_name => {
                            println!("navigate to dir: {}", dir_name);
                        }
                    },
                    "$ ls" => {
                        println!("list files");
                    }
                    _ => (),
                },
                _ => {
                    let size = l.split(' ').nth(0).unwrap();
                    let name = l.split(' ').nth(1).unwrap();

                    match size {
                        "dir" => {
                            println!("This is a dir with name {}", name);
                        }
                        _ => {
                            println!("This is a file with name {} and size {}", name, size);
                        }
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
