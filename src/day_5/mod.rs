use std::fs;
use std::collections::VecDeque;
use regex::Regex;

pub fn first_part() {
    let data = fs::read_to_string("data/day-5.txt").unwrap();
    let parts: Vec<&str> = data.split("\r\n\r\n").collect();
    let re_stack = Regex::new(r"((   )|(\[\w\])) ?").unwrap();
    let re_inst = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let stacks: &mut Vec<VecDeque<char>> = &mut Vec::new();

    for line in parts[0].split("\r\n") {
        if line.contains("1") {
            continue;
        }

        for i in re_stack.captures_iter(line).enumerate() {
            if stacks.len() <= i.0 {
                stacks.push(VecDeque::new());
            }
            
            if !i.1[1].trim().is_empty() {
                stacks[i.0].push_front(i.1[1].chars().nth(1).unwrap());
            }
        }
    }

    for line in parts[1].split("\r\n") {
        let cap = re_inst.captures(line).unwrap();

        let qty = cap[1].parse::<u32>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        for _i in 0..qty {
            let val = stacks.into_iter().nth(from).unwrap().pop_back().unwrap();
            stacks.into_iter().nth(to).unwrap().push_back(val);
        }
    }

    let res = stacks.into_iter().map(|s| s.pop_back().unwrap().to_string()).collect::<Vec<String>>().join("");
    println!("{}", res);
}

pub fn second_part () {
    let data = fs::read_to_string("data/day-5.txt").unwrap();
    let parts: Vec<&str> = data.split("\r\n\r\n").collect();
    let re_stack = Regex::new(r"((   )|(\[\w\])) ?").unwrap();
    let re_inst = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let stacks: &mut Vec<VecDeque<char>> = &mut Vec::new();

    for line in parts[0].split("\r\n") {
        if line.contains("1") {
            continue;
        }

        for i in re_stack.captures_iter(line).enumerate() {
            if stacks.len() <= i.0 {
                stacks.push(VecDeque::new());
            }
            
            if !i.1[1].trim().is_empty() {
                stacks[i.0].push_front(i.1[1].chars().nth(1).unwrap());
            }
        }
    }

    for line in parts[1].split("\r\n") {
        let cap = re_inst.captures(line).unwrap();

        let qty = cap[1].parse::<u32>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        let mut data: Vec<char> = vec![];

        for _i in 0..qty {
            data.push(stacks.into_iter().nth(from).unwrap().pop_back().unwrap());
        }

        data.reverse();

        for c in data {
            stacks.into_iter().nth(to).unwrap().push_back(c);
        }
    }

    let res = stacks.into_iter().map(|s| s.pop_back().unwrap().to_string()).collect::<Vec<String>>().join("");
    println!("{}", res);
}