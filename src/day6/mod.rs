use std::fs::read_to_string;

pub fn solve1() {
    let file_str = read_to_string("src/day6/input.txt").unwrap();
    let lines: Vec<&str> = file_str.lines().collect();

    let times: Vec<i32> = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let records: Vec<i32> = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut result: i32 = 1;

    for (time, record) in times.iter().zip(records) {
        let mut counter: i32 = 0;

        for i in 1..*time {
            if i * (*time - i) > record {
                counter += 1;
            }
        }

        result *= counter;
    }

    println!("{}", result);
}

pub fn solve2() {
    let file_str = read_to_string("src/day6/input.txt").unwrap();
    let lines: Vec<&str> = file_str.lines().collect();

    let time: i64 = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .concat()
        .parse::<i64>()
        .unwrap();

    let record: i64 = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .concat()
        .parse::<i64>()
        .unwrap();

    let mut result: i64 = 0;

    for i in 1..time {
        if i * (time - i) > record {
            result += 1;
        }
    }

    println!("{}", result);
}
