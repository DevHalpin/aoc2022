pub fn part1(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let shifts = line.split(',').collect::<Vec<_>>();
            let first_shift = shifts[0];
            let second_shift = shifts[1];

            let f_shift_numbers: Vec<u32> = first_shift
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let s_shift_numbers: Vec<u32> = second_shift
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            return f_shift_numbers[0] <= s_shift_numbers[0] && f_shift_numbers[1] >= s_shift_numbers[1]
                || s_shift_numbers[0] <= f_shift_numbers[0]
                    && s_shift_numbers[1] >= f_shift_numbers[1]
            
        })
        .count();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let shifts = line.split(',').collect::<Vec<_>>();
            let first_shift = shifts[0];
            let second_shift = shifts[1];

            let f_shift_numbers: Vec<u32> = first_shift
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let s_shift_numbers: Vec<u32> = second_shift
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            return (s_shift_numbers[0]..=s_shift_numbers[1]).contains(&f_shift_numbers[0])
                || (s_shift_numbers[0]..=s_shift_numbers[1]).contains(&f_shift_numbers[1])
                || (f_shift_numbers[0]..=f_shift_numbers[1]).contains(&s_shift_numbers[0])
                || (f_shift_numbers[0]..=f_shift_numbers[1]).contains(&s_shift_numbers[1]);
        })
        .count();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
