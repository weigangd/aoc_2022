use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let i = input.iter().position(|line| line.is_empty()).unwrap();
    let stacks = &input[..i - 1]; // skip numbering
    let commands = &input[i + 1..]; // skip empty line
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
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
