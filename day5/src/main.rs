use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let i = input.iter().position(|line| line.is_empty()).unwrap();
    let stacks = &input[..i - 1]; // skip numbering
    let stack_count = &input[i - 1].chars().filter(|c| c.is_numeric()).count();
    let commands = &input[i + 1..]; // skip empty line
    let mut stacks1 = get_stacks(*stack_count, stacks);
    move_elements(&mut stacks1, commands);
    get_result(&mut stacks1);
    let mut stacks2 = get_stacks(*stack_count, stacks);
    move_elements2(&mut stacks2, commands);
    get_result(&mut stacks2);
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

fn get_stacks(stack_count: usize, input: &[String]) -> Vec<Vec<char>> {
    let mut i = 1;
    let offset = 4;
    let mut stacks = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        let stack = input
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|c| c.is_alphabetic())
            .rev()
            .collect();
        stacks.push(stack);
        i += offset;
    }
    stacks
}

fn move_elements(stacks: &mut Vec<Vec<char>>, commands: &[String]) {
    for line in commands {
        let (n, from, to) = parse_command(line);
        //println!("n: {n}, from: {from}, to: {to}");
        for _ in 0..n {
            if let Some(element) = stacks[from as usize - 1].pop() {
                stacks[to as usize - 1].push(element);
            }
        }
    }
}

fn move_elements2(stacks: &mut Vec<Vec<char>>, commands: &[String]) {
    for line in commands {
        let (n, from, to) = parse_command(line);
        //println!("n: {n}, from: {from}, to: {to}");
        let mut tmp = Vec::new();
        for _ in 0..n {
            if let Some(element) = stacks[from as usize - 1].pop() {
                tmp.push(element);
            }
        }

        for element in tmp.into_iter().rev() {
            stacks[to as usize - 1].push(element);
        }
    }
}

fn parse_command(line: &str) -> (u32, u32, u32) {
    let s = line.replace("move ", "");
    let s = s.replace(" from ", ",");
    let s = s.replace(" to ", ",");
    let mut chars = s.split(",").map(|s| s.parse::<u32>().unwrap());
    (
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    )
}

fn get_result(stacks: &mut Vec<Vec<char>>) {
    println!("");
    for stack in stacks {
        print!("{}", stack.pop().unwrap())
    }
    println!("");
}

//#[cfg(test)]
//mod test {
//    use crate::{count_containing, count_overlapping, read_input};
//
//    #[test]
//    fn test_count_containing() {
//        let input = read_input("input/test1.txt");
//        let sum = count_containing(&input);
//        assert_eq!(sum, 2);
//    }
//
//    #[test]
//    fn test_count_overlapping() {
//        let input = read_input("input/test1.txt");
//        let sum = count_overlapping(&input);
//        assert_eq!(sum, 4);
//    }
//
//    //#[test]
//    //fn test_group_priority() {
//    //    let input = read_input("input/test1.txt");
//    //    let sum = sum_group_priority(&input);
//    //    assert_eq!(sum, 70);
//    //}
//}
