use std::fs;

fn calc_points(i: char) -> u32 {
    let mut val = i as u32;
    val -= if i.is_lowercase() { 96 } else { 38 };
    val
}

struct Sack {
    comp_1: String,
    comp_2: String
}

pub fn first_part() {
    let sacks: Vec<Sack> = fs::read_to_string("data/day-3.txt").unwrap().split("\r\n").map(|line| {
        let half_len = line.chars().count() / 2;

        Sack {
            comp_1: line.chars().take(half_len).collect(),
            comp_2: line.chars().skip(half_len).collect()
        }
    }).collect();

    let mut sum: u32 = 0;

    for sack in sacks {
        sum += sack.comp_1.chars().filter(|i| {
            sack.comp_2.contains(*i)
        }).map(|i| calc_points(i)).next().unwrap();
    }

    println!("Final: {}", sum);
}

pub fn second_part() {
    let data = fs::read_to_string("data/day-3.txt").unwrap();
    let mut sacks: Vec<&str> = data.split("\r\n").collect();
    let mut sum: u32 = 0;

    for chunk in sacks.chunks_mut(3) {
        chunk.sort_by(|a, b| a.len().cmp(&b.len()));

        for i in chunk[0].chars() {
            if chunk[1].contains(i) && chunk[2].contains(i) {
                sum += calc_points(i);
                break;
            }
        }
    }

    println!("{}", sum);
}
