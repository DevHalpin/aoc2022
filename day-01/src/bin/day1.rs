use std::fs;

fn part1(input: &str) -> String {
    let result = input.split("\n\n").map(|calories| {
        let total_sum = calories
            .lines()
            .map(|calorie| {
                let parsed_calorie = calorie.parse::<u32>().unwrap();
                parsed_calorie
            })
            .sum::<u32>();
        total_sum
    });

    let max_value = result.max().unwrap();

    max_value.to_string()
}

fn part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|calories| {
            let total_sum = calories
                .lines()
                .map(|calorie| {
                    let parsed_calorie = calorie.parse::<u32>().unwrap();
                    parsed_calorie
                })
                .sum::<u32>();
            total_sum
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();

    sum.to_string()
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}


