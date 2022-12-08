use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("input/real1.txt");
    let sum = count_visible(&input);
    println!("A: {sum}");
    let sum = get_max_scenic_score(&input);
    println!("B: {sum}");
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

fn count_visible(input: &[String]) -> u32 {
    let mut rows = Vec::with_capacity(input.len());
    for line in input {
        let trees: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(trees);
    }

    let mut count = 0;
    for (i, tree_row) in rows.iter().enumerate() {
        for (j, tree) in tree_row.iter().enumerate() {
            let row_visible = tree_row[..j].iter().all(|other| other < tree)
                || tree_row[(j + 1)..].iter().all(|other| other < tree);
            let column_visible = rows[..i].iter().all(|row| &row[j] < tree)
                || rows[(i + 1)..].iter().all(|row| &row[j] < tree);
            if row_visible || column_visible {
                count += 1;
            }
        }
    }

    count
}

fn get_max_scenic_score(input: &[String]) -> usize {
    let mut rows = Vec::with_capacity(input.len());
    for line in input {
        let trees: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(trees);
    }
    let mut max_score = 0;
    for (i, tree_row) in rows.iter().enumerate() {
        for (j, tree) in tree_row.iter().enumerate() {
            let mut score =
                get_directional_scenice_score(*tree, tree_row[..j].iter().copied().rev());
            score *= get_directional_scenice_score(*tree, tree_row[(j + 1)..].iter().copied());
            score *= get_directional_scenice_score(
                *tree,
                rows[..i].iter().map(|row| &row[j]).copied().rev(),
            );
            score *= get_directional_scenice_score(
                *tree,
                rows[(i + 1)..].iter().map(|row| &row[j]).copied(),
            );
            if max_score < score {
                max_score = score;
            }
        }
    }
    max_score
}

fn get_directional_scenice_score(tree: u32, direction: impl Iterator<Item = u32>) -> usize {
    let mut count = 0;
    for other in direction {
        count += 1;
        if other >= tree {
            break;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use crate::{count_visible, get_max_scenic_score, read_input};

    #[test]
    fn test_count_visible() {
        let input = read_input("input/test1.txt");
        let sum = count_visible(&input);
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_get_max_scenic_score() {
        let input = read_input("input/test1.txt");
        let sum = get_max_scenic_score(&input);
        assert_eq!(sum, 8);
    }
}
