use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let chars: Vec<char> = input[0].chars().collect();
    let value = get_start(&chars, 4);
    println!("A: {value}");
    let value = get_start(&chars, 14);
    println!("B: {value}");
}

fn get_start(chars: &[char], size: usize) -> usize {
    for (i, window) in chars.windows(size).enumerate() {
        let set: HashSet<char> = window.iter().copied().collect();
        if set.len() == size {
            return i + size;
        }
    }
    0
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}
