use std::fs;
use std::collections::HashSet;

pub fn detect_marker(data: &str, marker_len: usize) -> usize {
    for i in 0..(data.len() - marker_len) {
        let marker = &data[i..i + marker_len];
        let mut map: HashSet<char> = HashSet::new();
        marker.chars().for_each(|x| {
            map.insert(x);
        });

        if map.len() == marker_len {
            return i + marker_len;
        }
    }

    return 0;
}

pub fn first_part() {
    let data = fs::read_to_string("data/day-6.txt").unwrap();
    println!("{}", detect_marker(&data, 4));
}

pub fn second_part() {
    let data = fs::read_to_string("data/day-6.txt").unwrap();
    println!("{}", detect_marker(&data, 14));
}