use itertools::Itertools;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let max = get_max_calories(&input);
    println!("{max}");
    let top_3_sum = get_top_calories(3, &input);
    println!("{top_3_sum}");
}

fn get_max_calories(input: &[String]) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|split| split.iter().map(|v| v.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn get_top_calories(n: usize, input: &[String]) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|split| {
            split
                .iter()
                .map(|v| v.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .map(Reverse)
        .sorted()
        .take(n)
        .map(|v| v.0)
        .sum()
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{get_max_calories, get_top_calories, read_input};

    #[test]
    fn test_get_max_calories() {
        let input = read_input("input/test1.txt");
        let max = get_max_calories(&input);
        assert_eq!(max, 24000);
    }

    #[test]
    fn test_get_top_calories() {
        let input = read_input("input/test1.txt");
        let max = get_top_calories(3, &input);
        assert_eq!(max, 45000);
    }
}
