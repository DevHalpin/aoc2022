use std::collections::HashSet;

fn get_marker(input: &str, window_size: usize) -> String {
    let characters = input.chars().collect::<Vec<char>>();

    let iter = characters.windows(window_size).enumerate().find(|(_i,seq)| {
        let unique = seq.iter().collect::<HashSet<&char>>();
        seq.len() == unique.len()
    })
    .unwrap();

    (iter.0 + window_size).to_string()
}

pub fn part1(input: &str) -> String {
    get_marker(input, 4)
}

pub fn part2(input: &str) -> String {
    get_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "7"
        );
        assert_eq!(
            part1("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "5"
        );
        assert_eq!(
            part1("nppdvjthqldpwncqszvftbrmjlhg"),
            "6"
        );
        assert_eq!(
            part1(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
            ),
            "10"
        );
        assert_eq!(
            part1(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
            ),
            "11"
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "19"
        );
        assert_eq!(
            part2("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "23"
        );
        assert_eq!(
            part2("nppdvjthqldpwncqszvftbrmjlhg"),
            "23"
        );
        assert_eq!(
            part2(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
            ),
            "29"
        );
        assert_eq!(
            part2(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
            ),
            "26"
        );
    }
}
