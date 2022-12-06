use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real.txt");
    println!("Result A: {}", sum_score(&input));
    println!("Result B: {}", sum_score2(&input));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn get_winning_shape(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn get_losing_shape(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn get_planned_shape(&self, s: &str) -> Self {
        match s {
            "X" => self.get_losing_shape(),
            "Y" => *self,
            "Z" => self.get_winning_shape(),
            _ => panic!("Invalid input"),
        }
    }
}

impl From<&str> for Shape {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

fn sum_score(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|line| (Shape::from(&line[0..1]), Shape::from(&line[2..3])))
        .map(score)
        .sum()
}

fn sum_score2(lines: &[String]) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            let elf_shape = Shape::from(&line[0..1]);
            (elf_shape, elf_shape.get_planned_shape(&line[2..3]))
        })
        .map(score)
        .sum()
}

fn score(elf_my_shapes: (Shape, Shape)) -> i32 {
    let shape_score = elf_my_shapes.1 as i32;
    let outcome_score = match elf_my_shapes {
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (a, b) if a == b => 3,
        _ => 0,
    };

    outcome_score + shape_score
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{read_input, sum_score, sum_score2};

    #[test]
    fn test_sum_score() {
        let input = read_input("input/test1.txt");
        assert_eq!(sum_score(&input), 15);
    }

    #[test]
    fn test_sum_score2() {
        let input = read_input("input/test1.txt");
        assert_eq!(sum_score2(&input), 12);
    }
}
