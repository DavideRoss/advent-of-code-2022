use std::fs;
use regex::Regex;

pub fn first_part() {
    let data = fs::read_to_string("data/day-4.txt").unwrap();
    let lines: Vec<&str> = data.split("\r\n").collect();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut pairs = 0;

    for line in lines {
        for cap in re.captures_iter(line) {
            let e1p1 = cap[1].parse::<u32>().unwrap();
            let e1p2 = cap[2].parse::<u32>().unwrap();
            let e2p1 = cap[3].parse::<u32>().unwrap();
            let e2p2 = cap[4].parse::<u32>().unwrap();

            if (e2p1 >= e1p1 && e2p2 <= e1p2) || (e1p1 >= e2p1 && e1p2 <= e2p2) {
                pairs += 1;
            }
        }
    }

    println!("{}", pairs);
}

pub fn second_part() {
    let data = fs::read_to_string("data/day-4.txt").unwrap();
    let lines: Vec<&str> = data.split("\r\n").collect();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut pairs = 0;

    for line in lines {
        for cap in re.captures_iter(line) {
            let e1p1 = cap[1].parse::<u32>().unwrap();
            let e1p2 = cap[2].parse::<u32>().unwrap();
            let e2p1 = cap[3].parse::<u32>().unwrap();
            let e2p2 = cap[4].parse::<u32>().unwrap();

            if 
                (e1p2 >= e2p1 && e1p2 <= e2p2) ||
                (e1p1 >= e2p1 && e1p1 <= e2p2) ||
                (e2p2 >= e1p1 && e2p2 <= e1p2) ||
                (e2p1 >= e1p1 && e2p1 <= e1p2)
            {
                pairs += 1;
            }
        }
    }

    println!("{}", pairs);
}
