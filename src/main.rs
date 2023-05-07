use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// #[derive(Debug)]
// struct Node {
//     content: Option<HashMap<String, Node>>,
//     size: Option<i32>,
// }

mod AOC {
    use std::rc::Rc;

    pub struct MFile {
        name: String,
        fsize: i32,
    }

    pub struct MDir {
        name: String,
        contents: Vec<MNode>,
        parent: Option<Rc<MDir>>,
    }

    impl MDir {
        pub fn new(name: String) -> Self {
            MDir {
                name: name,
                contents: vec![],
                parent: None as Option<Rc<MDir>>,
            }
        }

        pub fn add_file(&mut self, name: String, size: i32) {
            let file = MFile {
                name: name,
                fsize: size,
            };
            let rc = Rc::new(file);
            self.contents.push(MNode::File(Rc::clone(&rc)));
        }

        pub fn ls(&self) -> String {
            self.contents
                .iter()
                .map(|n| match n {
                    MNode::File(f) => f.as_ref().name.clone(),
                    MNode::Dir(d) => d.as_ref().name.clone(),
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    pub enum MNode {
        File(Rc<MFile>),
        Dir(Rc<MDir>),
    }
}

// Nodes point to their children and their parent
// We need to have a pointer to the root node always
// We need to have a pointer to the current node

// cd / changes the current node pointer to the root node
// cd .. changes to the current node parent
// ls lists the current directories. It should add any that aren't children of the current node.

fn main() {

    // root.contents.push(MNode::File(Rc::new(MFile {
    //     name: "def".to_string(),
    //     fsize: 30,
    // })));

    // root.contents.push(MNode::Dir(Rc::new(MDir {
    //     name: "qrst".to_string(),
    //     contents: vec![],
    // })));

    // root.contents.push(MNode::Dir(Rc::new(MDir {
    //     name: "bvcx".to_string(),
    //     contents: vec![],
    // })));

    // if let MNode::Dir(dir_3) = &root.contents[3] {
    //     Rc::clone(dir_3).contents.push(MNode::File(Rc::new(MFile {
    //         name: "def".to_string(),
    //         fsize: 30,
    //     })));

    //     // dir_3.contents.push(MNode::File(MFile {
    //     //     name: "def".to_string(),
    //     //     fsize: 30,
    //     // }));
    // } else {
    //     panic!("Not a dir")
    // }
    // let root = Node::new();

    // if let Ok(lines) = read_lines("./src/day7.txt") {
    //     for line in lines {
    //         let l = line.unwrap();
    //         println!("{}", l);

    //         match l.as_bytes()[0] as char {
    //             '$' => match &l[..4] {
    //                 "$ cd" => match l.split(' ').nth(2).unwrap() {
    //                     "/" => {
    //                         println!("change to root");
    //                     }
    //                     ".." => {
    //                         println!("naviate out");
    //                     }
    //                     dir_name => {
    //                         println!("navigate to dir: {}", dir_name);
    //                     }
    //                 },
    //                 "$ ls" => {
    //                     println!("list files");
    //                 }
    //                 _ => (),
    //             },
    //             _ => {
    //                 let size = l.split(' ').nth(0).unwrap();
    //                 let name = l.split(' ').nth(1).unwrap();

    //                 match size {
    //                     "dir" => {
    //                         println!("This is a dir with name {}", name);
    //                     }
    //                     _ => {
    //                         println!("This is a file with name {} and size {}", name, size);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::AOC::*;

    #[test]
    fn supports_adding_files_to_a_dir() {
        let mut root = MDir::new("/".to_string());

        root.add_file("abcd".to_string(), 30);
        root.add_file("defg".to_string(), 30);
        root.add_file("hijk".to_string(), 30);
        root.add_file("lmno".to_string(), 30);

        assert_eq!("abcd\ndefg\nhijk\nlmno", root.ls())
    }

    // #[test]
    // fn dir_size_computes_size_of_all_containing_elements() {
    //     let mut root = MDir {
    //         name: "/".to_string(),
    //         contents: vec![],
    //         parent: None,
    //     };

    //     assert_eq!(30, root.size());
    // }
}
