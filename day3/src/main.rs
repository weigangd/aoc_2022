use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let sum = sum_priority(&input);
    println!("A: {sum}");
    let sum = sum_group_priority(&input);
    println!("B: {sum}");
}

struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn get_duplicate(&self) -> char {
        self.compartment2
            .chars()
            .find(|c| self.compartment1.contains(*c))
            .unwrap()
    }

    fn priority(&self) -> i32 {
        priority(self.get_duplicate())
    }

    fn get_disinct_values(&self) -> HashSet<char> {
        let mut result = HashSet::new();
        for c in self.compartment1.chars() {
            result.insert(c);
        }
        for c in self.compartment2.chars() {
            result.insert(c);
        }
        result
    }

    fn get_common(&self, others: impl IntoIterator<Item = Self>) -> char {
        let mut chars = self.get_disinct_values();
        for rucksack in others {
            let other_chars = rucksack.get_disinct_values();
            chars.retain(|c| other_chars.contains(c))
        }
        *chars.iter().next().unwrap()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (compartment1, compartment2) = s.split_at(s.len() / 2);
        Self {
            compartment1: compartment1.to_owned(),
            compartment2: compartment2.to_owned(),
        }
    }
}

fn priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 'A' as i32 + 27
    } else if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        panic!("Invalid char");
    }
}

fn sum_priority(input: &[String]) -> i32 {
    input
        .iter()
        .map(|line| Rucksack::from(line.as_str()).priority())
        .sum()
}

fn sum_group_priority(input: &[String]) -> i32 {
    input
        .iter()
        .map(|line| Rucksack::from(line.as_str()))
        .chunks(3)
        .into_iter()
        .map(|mut chunks| priority(chunks.next().unwrap().get_common(chunks)))
        .sum()
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{read_input, sum_group_priority, sum_priority};

    #[test]
    fn test_sum_priority() {
        let input = read_input("input/test1.txt");
        let sum = sum_priority(&input);
        assert_eq!(sum, 157);
    }

    #[test]
    fn test_group_priority() {
        let input = read_input("input/test1.txt");
        let sum = sum_group_priority(&input);
        assert_eq!(sum, 70);
    }
}
