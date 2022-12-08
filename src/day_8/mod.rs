use std::fs;
use std::ops::Range;

pub fn first_part() {
    let data = fs::read_to_string("data/demo.txt").unwrap();
    let lines = data.split("\r\n").into_iter();

    let mut trees: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        trees.push(line.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect());
    }

    // println!("{:#?}", trees[1]);

    let cols = trees.len();
    let rows = trees[0].len();

    let mut visible = cols * 2 + rows * 2 - 4;

    for y in 1..(cols - 1) {
        for x in 1..(rows - 1) {
            let curr = trees[y][x];
            let mut is_visible = true;
            println!("({}, {}) => {}", x, y, curr);

            // for cy in (0..y).rev() {
            //     // print!("Check 1: {} |= {} <= {} ({})", is_visible, trees[cy][x], curr, trees[cy][x] <= curr);
            //     is_visible &= trees[cy][x] <= curr;
            //     // println!(" => {}", is_visible);
            // }

            // println!("C1 => {}", is_visible);

            // for cy in (y + 1)..rows {
            //     is_visible &= trees[cy][x] <= curr;
            // }

            // println!("C2 => {}", is_visible);

            for cx in (0..x).rev() {
                print!("Check 1: {} |= {} <= {} ({})", is_visible, trees[y][cx], curr, trees[y][cx] <= curr);
                is_visible &= trees[y][cx] <= curr;
                println!(" => {}", is_visible);
            }

            println!("C3 => {}", is_visible);

            for cx in (x + 1)..cols {
                print!("Check 1: {} |= {} <= {} ({})", is_visible, trees[y][cx], curr, trees[y][cx] <= curr);
                is_visible &= trees[y][cx] <= curr;
                println!(" => {}", is_visible);
            }

            println!("C4 => {}", is_visible);
            println!("================");

            if is_visible {
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}