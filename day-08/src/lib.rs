pub fn part1(input: &str) -> String {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|char| char as u32).collect())
        .collect();
    let width = lines.last().unwrap().len();
    let height = lines.len();

    let mut non_visible_trees = 0;
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let current_tree_height = lines[x][y];
            // look left
            if lines[x][0..y].iter().any(|tree_height| *tree_height >= current_tree_height)  
            // look right
                && lines[x][(y + 1)..].iter().any(|tree_height| *tree_height >= current_tree_height)
            // look down
                && lines[0..x].iter().any(|row| row[y] >= current_tree_height)
            // look up
                && lines[(x + 1)..].iter().any(|row| row[y] >= current_tree_height)
            {
                non_visible_trees += 1;
            }
        }
    }

    let visible_trees = width * height - non_visible_trees;

    visible_trees.to_string()
}

pub fn part2(input: &str) -> String {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|char| char as u32).collect())
        .collect();
    let width = lines.last().unwrap().len();
    let height = lines.len();

    let mut max_score = 0;

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let current_tree_height = lines[x][y];
            let mut left_score = 0;
            let mut right_score = 0;
            let mut up_score = 0;
            let mut down_score = 0;

            // look left
           let left = lines[x][0..y]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, f)| **f >= current_tree_height);

            match left {
                Some(left) => left_score += left.0 + 1,
                None => left_score += y,
            }

            //look right
            let right = lines[x][(y + 1)..]
                .iter()
                .enumerate()
                .find(|(_, f)| **f >= current_tree_height);

            match right {
                Some(right) => right_score += right.0 + 1,
                None => right_score += width - y - 1,
            }

            let down = lines[0..x]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, row)| row[y] >= current_tree_height);

            match down {
                Some(down) => down_score += down.0 + 1,
                None => down_score += x,
            }

            let up = lines[(x + 1)..]
                .iter()
                .enumerate()
                .find(|(_, row)| row[y] >= current_tree_height);

            match up {
                Some(up) => up_score += up.0 + 1,
                None => up_score += height - x -1 ,
            }

            // dbg!(left_score, right_score, up_score, down_score);

            let score = left_score * right_score * up_score * down_score;

            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    // #[test]
    // fn part1_works() {
    //     let result = part1(INPUT);
    //     assert_eq!(result, "21");
    // }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "8");
    }
}
