use std::fs;

pub fn first_part() {
    let data = fs::read_to_string("data/day-1.txt").unwrap();
    let lines = data.split("\n");

    let mut current = 0;
    let mut max = 0;
    
    for line in lines {
        if line.trim().is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            current += line.trim().parse::<i32>().unwrap();
        }
    }

    println!("Max: {}", max);
}

pub fn second_part() {
    let mut elves: Vec<i32> = fs::read_to_string("data/day-1.txt").unwrap().split("\r\n\r\n").map(|elf| {
        elf.split("\r\n").map(|x| x.trim().parse::<i32>().unwrap()).sum()
    }).collect();

    elves.sort();
    let test: i32 = elves.into_iter().rev().take(3).sum();

    println!("{}", test);
}