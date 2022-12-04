use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, val)| (val, index + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[0..sack_length];
            let compartment_b = &line[sack_length..(sack_length * 2)];

            let common = compartment_a
                .chars()
                .find(|char| compartment_b.contains(*char))
                .unwrap();

            scores.get(&common).unwrap()
        })
        .sum::<usize>();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, val)| (val, index + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .tuples::<(_, _, _)>()
        .into_iter()
        .map(|(i1, i2, i3)| {
            dbg!((i1, i2, i3));
            let common = i1
                .chars()
                .find(|char| i2.contains(*char) && i3.contains(*char))
                .unwrap();
            scores.get(&common).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "70");
    }
}
