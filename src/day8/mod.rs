use std::collections::HashMap;
use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let file_str = read_to_string("src/day8/input.txt").unwrap();

    let mut contacts: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut lines = file_str.lines();
    let commands: Vec<char> = lines.next().unwrap().chars().collect();

    for line in lines.skip(1) {
        let (key, pair) = line.split_once(" = (").unwrap();
        let (left, right) = pair.trim_end_matches(')').split_once(", ").unwrap();
        contacts.insert(key, (left, right));
    }

    let mut curent: Option<&(&str, &str)> = contacts.get("AAA");

    let mut i = 0;
    let mut result: i32 = 1;

    loop {
        if i == commands.len() {
            i = 0;
        }

        let new_element: &str = if commands[i] == 'L' {
            curent.unwrap().0
        } else if commands[i] == 'R' {
            curent.unwrap().1
        } else {
            panic_any("wrong command")
        };

        if new_element == "ZZZ" {
            break;
        }
        curent = contacts.get(new_element);

        i += 1;
        result += 1;
    }

    println!("{}", result);
}

// not correct solution.
// it is needed to use cycles + lcm
pub fn solve2() {
    let file_str = read_to_string("src/day8/input.txt").unwrap();

    let mut contacts: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut lines = file_str.lines();
    let commands: Vec<char> = lines.next().unwrap().chars().collect();

    for line in lines.skip(1) {
        let (key, pair) = line.split_once(" = (").unwrap();
        let (left, right) = pair.trim_end_matches(')').split_once(", ").unwrap();
        contacts.insert(key, (left, right));
    }

    let mut currents: Vec<Option<&(&str, &str)>> = Vec::new();

    /*
    BGA
    XJA
    JNA
    AAA
    PTA
    SLA
        **/

    for (key, value) in &contacts {
        if key.ends_with('A') {
            println!("{}", key);
            currents.push(Some(value))
        }
    }

    let mut i = 0;
    let mut result: i64 = 1;

    loop {
        if i == commands.len() {
            i = 0;
        }

        let new_elements: Vec<&str> = if commands[i] == 'L' {
            currents.iter().map(|s| s.unwrap().0).collect()
        } else if commands[i] == 'R' {
            currents.iter().map(|s| s.unwrap().1).collect()
        } else {
            panic_any("wrong command")
        };

        if new_elements.iter().all(|s| s.ends_with(&"Z")) {
            break;
        }

        currents = new_elements.iter().map(|el| contacts.get(el)).collect();

        i += 1;
        result += 1;

        if result % 10_000_000 == 0 {
            println!("{}", result);
        }
    }

    println!("{}", result);
}
