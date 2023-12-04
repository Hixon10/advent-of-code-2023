use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn solve1() {
    let file_str = read_to_string("src/day4/input.txt").unwrap();

    let mut result: i32 = 0;

    for line in file_str.lines() {
        let v: Vec<&str> = line.split(':').collect();
        let parts: Vec<&str> = v[1].split('|').collect();

        let win: HashSet<i32> = HashSet::from_iter(
            parts[0]
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );

        let count = parts[1]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|x| win.contains(x))
            .count();

        if count > 0 {
            result += 2i32.pow((count - 1) as u32)
        }
    }

    println!("{}", result)
}

pub fn solve2() {
    let file_str = read_to_string("src/day4/input.txt").unwrap();
    let lines: Vec<&str> = file_str.lines().collect();

    let mut contacts: HashMap<usize, i32> = HashMap::new();
    for i in 0..lines.len() {
        contacts.insert(i, 1);
    }

    for (i, line) in lines.iter().enumerate() {
        let v: Vec<&str> = line.split(':').collect();
        let parts: Vec<&str> = v[1].split('|').collect();

        let win: HashSet<i32> = HashSet::from_iter(
            parts[0]
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );

        let count = parts[1]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|x| win.contains(x))
            .count();

        if count > 0 {
            let current_number = contacts[&i];
            let current_win = current_number;

            for j in (i + 1)..(i + 1 + count) {
                let next_card: Option<&i32> = contacts.get(&j);
                match next_card {
                    None => {}
                    Some(nc) => {
                        contacts.insert(j, nc + current_win);
                    }
                }
            }
        }
    }

    let mut result: i32 = 0;

    for i in 0..lines.len() {
        let val = contacts[&i];
        result += val;
    }

    println!("{}", result)
}
