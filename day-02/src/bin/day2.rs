use std::fs;

fn part1(input: &str) -> String {
    let mut moves: Vec<(char, char)> = Vec::new();
    let mut score = 0;
    let result = input.lines();
    for line in result {
        let (opp, me) = line.split_once(' ').unwrap();
        moves.push((
            opp.chars().take(1).next().unwrap(),
            me.chars().take(1).next().unwrap(),
        ))
    }

    for m in moves {
        let my_move = match m.1 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Unexpected value!"),
        };
        let opp_move = match m.0 {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Unexpected value!"),
        };

        score += my_move;
        if my_move == opp_move {
            score += 3
        } else if my_move == 1 && opp_move == 3
            || my_move == 2 && opp_move == 1
            || my_move == 3 && opp_move == 2
        {
            score += 6
        }
    }

    score.to_string()
}

fn part2(input: &str) -> String {
    let mut moves: Vec<(char, char)> = Vec::new();
    let mut score = 0;
    let result = input.lines();
    for line in result {
        let (opp, me) = line.split_once(' ').unwrap();
        moves.push((
            opp.chars().take(1).next().unwrap(),
            me.chars().take(1).next().unwrap(),
        ))
    }

    for m in moves {
        let opp_move = match m.0 {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Unexpected value!"),
        };
        let my_score = match m.1 {
            'X' => {
                if opp_move == 1 {
                    3
                } else {
                    opp_move - 1
                }
            }
            'Y' => 3 + opp_move,
            'Z' => 6 + if opp_move == 3 { 1 } else { opp_move + 1 },
            _ => panic!("Unexpected value!"),
        };
        score += my_score
    }

    score.to_string()
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}