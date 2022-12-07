use std::fs;

#[derive(Debug)]
struct Element {
    name: String,
    size: usize,
    elements: Vec<Element>,
    parent: Option<Box<Element>>
}

pub fn first_part() {
    let data = fs::read_to_string("data/demo.txt").unwrap();
    let mut lines = data.split("\r\n");
    lines.next(); // Skip first line

    let mut current: Element = Element {
        name: String::from("/"),
        size: 0,
        elements: Vec::new(),
        parent: None
    };

    for line in lines {
        match &line[0..1] {
            "$" => {
                // Command
                let cmd = &line[2..4];

                match cmd {
                    "cd" => {
                        let new_cd = &line[5..];
                        println!("Request CD => {}", new_cd);
                        current = current.elements.into_iter().find(|x| x.name == new_cd).unwrap();
                    },
                    "ls" => {},
                    _ => println!("Unknown command: {}", cmd)
                }
            },

            "d" => {
                // Directory
                let dir_name = &line[4..];
                current.elements.push(Element {
                    name: String::from(dir_name),
                    size: 0,
                    elements: Vec::new(),
                    parent: Some(Box::new(current))
                });
            },

            _ => {
                // File
            }
        }
    }

    println!("{:#?}", current);
}