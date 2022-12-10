use std::{
    fs::File,
    io::{BufReader, Lines},
};

fn get_directory_size(input: &mut Lines<BufReader<File>>, list: &mut Vec<i32>) -> i32 {
    let mut size = 0;
    loop {
        let next_line = input.next();
        match next_line {
            None => break,
            _ => {
                let line = next_line.unwrap().unwrap();
                if line == "$ cd .." {
                    break;
                }
                let words: Vec<&str> = line.split(" ").collect();
                if words[1] == "cd" {
                    size += get_directory_size(input, list);
                }
                size += words[0].parse::<i32>().unwrap_or(0);
            }
        };
    }
    list.push(size);

    size
}

pub fn part1(input: &mut Lines<BufReader<File>>) -> String {
    let mut directories: Vec<i32> = Vec::new();
    let _total = get_directory_size(input, &mut directories);
    let total_directories =
        directories
            .iter()
            .fold(0, |acc, dir| if *dir <= 100000 { acc + *dir } else { acc });
    total_directories.to_string()
}

pub fn part2(input: &mut Lines<BufReader<File>>) -> String {
    let mut directories: Vec<i32> = Vec::new();
    let total_directory_space = get_directory_size(input, &mut directories);
    let total_space = 70000000;
    let unused_space = 30000000;
    let needed_space = unused_space - (total_space - total_directory_space);
    let total_directories = directories.iter().fold(total_directory_space, |acc, dir| {
        if *dir <= acc && *dir >= needed_space {
            *dir
        } else {
            acc
        }
    });
    total_directories.to_string()
    // "nothing".to_string()
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;

    #[test]
    fn part1_works() {
        let file = File::open("./example.txt").unwrap();
        let mut reader = BufReader::new(file).lines();
        let result = part1(&mut reader);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let file = File::open("./example.txt").unwrap();
        let mut reader = BufReader::new(file).lines();
        let result = part2(&mut reader);
        assert_eq!(result, "24933642");
    }
}
