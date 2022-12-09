use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Dir {
    pub subdirs: HashMap<String, Dir>,
    pub files: Vec<DataFile>,
}

impl Dir {
    fn empty() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        let mut size: usize = self.files.iter().map(|file| file.size).sum();
        size += self.subdirs.values().map(|dir| dir.size()).sum::<usize>();
        size
    }
}

#[derive(Debug)]
struct DataFile {
    pub name: String,
    pub size: usize,
}

fn parse_dirs(input: &[String], pos: &mut usize, dir: &mut Dir) {
    while *pos < input.len() {
        let line = &input[*pos];
        *pos += 1;
        let mut splits = line.split(' ');
        match splits.next().unwrap() {
            "$" => match splits.next().unwrap() {
                "cd" => match splits.next().unwrap() {
                    ".." => return,
                    dir_name => parse_dirs(&input, pos, dir.subdirs.get_mut(dir_name).unwrap()),
                },
                "ls" => continue,
                _ => panic!("Wrong 2nd"),
            },
            "dir" => {
                dir.subdirs
                    .insert(splits.next().unwrap().to_string(), Dir::empty());
            }
            size => {
                let size = size.parse().unwrap();
                let name = splits.next().unwrap().to_string();
                let file = DataFile { size, name };
                dir.files.push(file);
            }
        }
    }
}

fn add_sum(dir: &Dir, sum: &mut usize) {
    for subdir in dir.subdirs.values() {
        let size = subdir.size();
        if size <= 100000 {
            *sum += size;
        }
        add_sum(subdir, sum)
    }
}

fn get_min_size(dir: &Dir, space_needed: usize) -> usize {
    let Some(mut potential) = dir
        .subdirs
        .values()
        .map(|dir| dir.size())
        .filter(|size| *size >= space_needed)
        .min() else {
            return dir.size()
        };

    for subdir in dir.subdirs.values() {
        let size = subdir.size();
        if size >= space_needed {
            let s = get_min_size(subdir, space_needed);
            if s < potential {
                potential = s;
            }
        }
    }
    potential
}

fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect::<Result<_, _>>().unwrap()
}

fn main() {
    let input = read_input("input/real1.txt");
    let mut root = Dir::empty();
    root.subdirs.insert("/".to_string(), Dir::empty());
    let mut pos = 0;
    parse_dirs(&input, &mut pos, &mut root);
    let mut sum = 0;
    add_sum(&root, &mut sum);
    println!("A: {sum}");
    let space_needed = 30000000 - (70000000 - root.size());
    let min_size = get_min_size(&root, space_needed);
    println!("B: {min_size}");
}
