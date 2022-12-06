use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let sum = count_containing(&input);
    println!("A: {sum}");
    let sum = count_overlapping(&input);
    println!("B: {sum}");
}

struct ElfPair {
    s0: (i32, i32),
    s1: (i32, i32),
}

impl ElfPair {
    fn containing_tuple(&self) -> Option<(i32, i32)> {
        match self {
            Self { s0, s1 } if s0.0 <= s1.0 && s0.1 >= s1.1 => Some(*s0),
            Self { s0, s1 } if s1.0 <= s0.0 && s1.1 >= s0.1 => Some(*s1),
            _ => None,
        }
    }

    fn is_overlapping(&self) -> bool {
        let Self { s0, s1 } = self;
        s0.0 <= s1.0 && s1.0 <= s0.1 || s1.0 <= s0.0 && s0.0 <= s1.1
    }
}

impl From<&String> for ElfPair {
    fn from(s: &String) -> Self {
        let elf_pair = s.split_once(',').unwrap();
        let (s0, e0) = elf_pair.0.split_once('-').unwrap();
        let (s1, e1) = elf_pair.1.split_once('-').unwrap();
        Self {
            s0: (s0.parse().unwrap(), e0.parse().unwrap()),
            s1: (s1.parse().unwrap(), e1.parse().unwrap()),
        }
    }
}

fn count_containing(input: &[String]) -> usize {
    input
        .iter()
        .map(ElfPair::from)
        .filter(|p| p.containing_tuple().is_some())
        .count()
}

fn count_overlapping(input: &[String]) -> usize {
    input
        .iter()
        .map(ElfPair::from)
        .filter(|p| p.is_overlapping())
        .count()
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{count_containing, count_overlapping, read_input};

    #[test]
    fn test_count_containing() {
        let input = read_input("input/test1.txt");
        let sum = count_containing(&input);
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_count_overlapping() {
        let input = read_input("input/test1.txt");
        let sum = count_overlapping(&input);
        assert_eq!(sum, 4);
    }

    //#[test]
    //fn test_group_priority() {
    //    let input = read_input("input/test1.txt");
    //    let sum = sum_group_priority(&input);
    //    assert_eq!(sum, 70);
    //}
}
