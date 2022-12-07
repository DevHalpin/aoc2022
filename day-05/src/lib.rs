use std::collections::VecDeque;


struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn part1(input: &str) -> String {
    let mut instructions = Vec::new();
    let mut ship = Vec::new();

    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        if words[0] == "move" {
            let amount = words[1].parse().unwrap();
            let from = words[3].parse::<usize>().unwrap() - 1;
            let to = words[5].parse::<usize>().unwrap() - 1;
            instructions.push(Move { amount, from, to });
        } else if line.contains('[') {
            for krate in line
                .chars()
                .enumerate()
                .filter(|(index, char)| *char != ' ' && (index + 3) % 4 == 0)
            {
                let crate_stack = (krate.0 - 1) / 4;
                while ship.len() < crate_stack + 1 {
                    ship.push(VecDeque::new());
                }
                ship[crate_stack].push_front(krate.1);
            }
        }
    }

    for i in instructions {
        for _k in 0..i.amount {
            let popped_crate = ship[i.from].pop_back().unwrap();
            ship[i.to].push_back(popped_crate);
        }
    }
 
    let mut result = "".to_string();
    for mut crate_stack in ship {
        result.push(crate_stack.pop_back().unwrap())
    }

    result.to_string()
}

pub fn part2(input: &str) -> String {

    let mut instructions = Vec::new();
    let mut ship = Vec::new();

    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        if words[0] == "move" {
            let amount = words[1].parse().unwrap();
            let from = words[3].parse::<usize>().unwrap() - 1;
            let to = words[5].parse::<usize>().unwrap() - 1;
            instructions.push(Move { amount, from, to });
        } else if line.contains('[') {
            for krate in line
                .chars()
                .enumerate()
                .filter(|(index, char)| *char != ' ' && (index + 3) % 4 == 0)
            {
                let crate_stack = (krate.0 - 1) / 4;
                while ship.len() < crate_stack + 1 {
                    ship.push(VecDeque::new());
                }
                ship[crate_stack].push_front(krate.1);
            }
        }
    }

    for i in instructions {
       let split = ship[i.from].len() - i.amount;
       let mut removed_array = ship[i.from].split_off(split);
       ship[i.to].append(&mut removed_array)
    }
 
    let mut result = "".to_string();
    for mut crate_stack in ship {
        result.push(crate_stack.pop_back().unwrap())
    }

    result.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
