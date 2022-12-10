use day_07::part1;
use day_07::part2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let mut reader = BufReader::new(file).lines();
    let file2 = File::open("./input.txt").unwrap();
    let mut reader2 = BufReader::new(file2).lines();
    println!("Part 1: {}", part1(&mut reader));
    println!("Part 2: {}", part2(&mut reader2));
}
