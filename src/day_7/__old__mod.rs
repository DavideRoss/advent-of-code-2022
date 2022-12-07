use std::fs;
use std::collections::{HashMap, VecDeque};
use std::fmt;

use regex::Regex;

struct Folder {
    size: usize,
    subfolders: Vec<String>
}

impl fmt::Debug for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Folder")
            .field("Size", &self.size)
            .field("Subfolders", &self.subfolders)
            .finish()
    }
}

pub fn first_part() {
    let data = fs::read_to_string("data/day-7.txt").unwrap();
    let lines = data.split("\r\n");

    let mut fs: HashMap<String, Folder> = HashMap::new();
    let mut path: VecDeque<String> = VecDeque::new();

    fs.insert(String::from("//"), Folder {
        size: 0,
        subfolders: vec![]
    });

    let re_file = Regex::new(r"^(\d+)").unwrap();

    for line in lines {
        match &line[0..1] {
            "$" => {
                let cmd = &line[2..4];
                
                match cmd {
                    "cd" => {
                        let new_cd = &line[5..];

                        if new_cd == ".." {
                            path.pop_back();
                        } else {
                            path.push_back(String::from(new_cd));
                        }
                    },
                    "ls" => {},
                    _ => println!("Unknown command: {}", line)
                }
            },
            "d" => {
                let dir_name = &line[4..];
                let cd = path.iter().fold(String::from(""), |acc, x| acc + x + "/");
                let folder = fs.get_mut(&cd).unwrap();
                folder.subfolders.push(String::from(dir_name));

                let full_path = path.iter().fold(String::new(), |acc, x| acc + x + "/") + dir_name + "/";

                fs.insert(String::from(full_path), Folder {
                    size: 0,
                    subfolders: Vec::new()
                });
            },
            _ => {
                let file_size = re_file.captures(line).unwrap()[1].parse::<usize>().unwrap();

                let mut path_copy = path.clone();
                while let Some(_f) = path_copy.pop_back() {
                    let full_path = path_copy.iter().fold(String::new(), |acc, x| acc + x + "/");

                    if full_path.trim().is_empty() {
                        break;
                    }

                    let folder = fs.get_mut(&full_path).unwrap();
                    folder.size += file_size;
                }
            }
        }
    }

    println!("{:#?}", fs);

    let mut total = 0;
    for (_path, folder) in fs.iter() {
        if folder.size < 100_000 {
            total += folder.size;
        }
    }

    println!("{}", total)
}