use std::fs;

/*

A X 1 Rock
B Y 2 Paper
C Z 3 Scissor

0 lose, 3 draw, 6 win

*/

pub fn first_part() {
    let elf_moves = vec!['A', 'B', 'C'];
    let me_moves = vec!['X', 'Y', 'Z'];

    let points: usize = fs::read_to_string("data/day-2.txt").unwrap().split("\r\n").map(|game| {
        let elf = elf_moves.iter().position(|&x| x == game.chars().nth(0).unwrap()).unwrap();
        let me = me_moves.iter().position(|&x| x == game.chars().nth(2).unwrap()).unwrap();

        let mut points = me + 1;

        if elf == me {
            points += 3;
        }

        if (elf == 0 && me == 1) || (elf == 1 && me == 2) || (elf == 2 && me == 0) {
            points += 6;
        }

        return points;
    }).sum();

    println!("{}", points);
}

/*

X Lose
Y Draw
Z Win

*/

pub fn second_part() {
    let moves = vec!['A', 'B', 'C'];
    let lose_moves = vec![3, 1, 2];
    let win_moves = vec![2, 3, 1];

    let points: usize = fs::read_to_string("data/day-2.txt").unwrap().split("\r\n").map(|game| {
        let elf = moves.iter().position(|&x| x == game.chars().nth(0).unwrap()).unwrap();
        let inst = game.chars().nth(2).unwrap();
        let mut points = 0;

        if inst == 'X' {
            points += lose_moves[elf];
        } else if inst == 'Y' {
            points += 4 + elf;
        } else {
            points += 6 + win_moves[elf];
        }

        return points;
    }).sum();

    println!("{}", points);
}
