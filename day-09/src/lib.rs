use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_position(
    dir: Direction,
    rope: &mut Vec<(i32, i32)>,
    positions: &mut HashMap<(i32, i32), i32>,
) {
    let direction = match dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    // Move head
    rope[0].0 += direction.0;
    rope[0].1 += direction.1;

    // Move rest of body
    for i in 1..rope.len() {
        let row_diff = rope[i - 1].0 - rope[i].0;
        let column_diff = rope[i - 1].1 - rope[i].1;

        if row_diff.abs() > 1 || column_diff.abs() > 1 {
            rope[i].0 += row_diff.signum();
            rope[i].1 += column_diff.signum();
        }
    }

    // Input new position visited by tail
    *positions.entry(rope[rope.len() - 1]).or_insert(0) += 1;
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    let movement_vec: Vec<(Direction, i32)> = input
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_once(' ').unwrap();
            let parse_dist = dist.parse::<i32>().unwrap();
            let movement = match dir {
                "U" => (Direction::Up, parse_dist),
                "D" => (Direction::Down, parse_dist),
                "L" => (Direction::Left, parse_dist),
                "R" => (Direction::Right, parse_dist),
                _ => panic!("Invalid direction ' {dir}'"),
            };
            movement
        })
        .collect();
    movement_vec
}

pub fn part1(input: &str) -> String {
    let movements = parse_input(input);
    let mut positions = HashMap::new();

    let mut rope = vec![(0, 0); 2];

    for (dir, amount) in movements {
        for _ in 0..amount {
            update_position(dir, &mut rope, &mut positions)
        }
    }

    dbg!(&positions);

    positions.len().to_string()
}

pub fn part2(input: &str) -> String {
    let movements = parse_input(input);
    let mut positions = HashMap::new();

    let mut rope = vec![(0, 0); 10];

    for (dir, amount) in movements {
        for _ in 0..amount {
            update_position(dir, &mut rope, &mut positions)
        }
    }

    dbg!(&positions);

    positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "1");
    }
}
