use std::fs;
use day_09::part1;
use day_09::part2;




fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}


