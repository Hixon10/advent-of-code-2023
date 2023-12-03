use std::fs::read_to_string;

pub fn solve1() {
    let file_str = read_to_string("src/day3/input.txt").unwrap();

    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in file_str.lines() {
        vec.push(line.chars().collect())
    }

    let mut result: i32 = 0;

    for i in 0..vec.len() {
        let width = vec.get(i).unwrap().len();
        for j in 0..width {
            let ch = vec.get(i).unwrap().get(j).unwrap();
            if ch.is_ascii_digit() || ch.eq(&'.') {
                continue;
            }

            match vec.get_mut(i - 1) {
                None => {}
                Some(prev_line) => {
                    let x: i32 = calculate_line(j, prev_line).iter().sum();
                    result += x;
                }
            }

            let start_index: Option<usize> = process_left(vec.get(i).unwrap(), j);
            match start_index {
                None => {}
                Some(start) => {
                    let current_number = get_number(vec.get_mut(i).unwrap(), start, j);
                    result += current_number;
                    println!("{}", current_number);
                }
            }

            let end_index: Option<usize> = process_right(vec.get(i).unwrap(), j);
            match end_index {
                None => {}
                Some(end) => {
                    let current_number = get_number(vec.get_mut(i).unwrap(), j + 1, end + 1);
                    result += current_number;
                    println!("{}", current_number);
                }
            }

            match vec.get_mut(i + 1) {
                None => {}
                Some(prev_line) => {
                    let x: i32 = calculate_line(j, prev_line).iter().sum();
                    result += x;
                }
            }
        }
    }

    println!("{}", result)
}

pub fn solve2() {
    let file_str = read_to_string("src/day3/input.txt").unwrap();

    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in file_str.lines() {
        vec.push(line.chars().collect())
    }

    let mut result = 0;

    for i in 0..vec.len() {
        let width = vec.get(i).unwrap().len();
        for j in 0..width {
            let ch = vec.get(i).unwrap().get(j).unwrap();
            if !ch.eq(&'*') {
                continue;
            }

            let mut numbers: Vec<i32> = Vec::new();

            match vec.get_mut(i - 1) {
                None => {}
                Some(prev_line) => {
                    for x in calculate_line(j, prev_line) {
                        numbers.push(x);
                    }
                }
            }

            let start_index: Option<usize> = process_left(vec.get(i).unwrap(), j);
            match start_index {
                None => {}
                Some(start) => {
                    let current_number = get_number(vec.get_mut(i).unwrap(), start, j);
                    numbers.push(current_number);
                }
            }

            let end_index: Option<usize> = process_right(vec.get(i).unwrap(), j);
            match end_index {
                None => {}
                Some(end) => {
                    let current_number = get_number(vec.get_mut(i).unwrap(), j + 1, end + 1);
                    numbers.push(current_number);
                }
            }

            match vec.get_mut(i + 1) {
                None => {}
                Some(prev_line) => {
                    for x in calculate_line(j, prev_line) {
                        numbers.push(x);
                    }
                }
            }

            if numbers.len() == 2 {
                result += numbers[0] * numbers[1]
            }
        }
    }

    println!("{}", result)
}

fn calculate_line(j: usize, line: &mut Vec<char>) -> Vec<i32> {
    // let mut result = 0;
    let mut vec: Vec<i32> = Vec::new();

    let start_index: Option<usize> = process_left(line, j);
    let end_index: Option<usize> = process_right(line, j);
    let has_up = line.get(j).unwrap().is_ascii_digit();

    if start_index.is_some() && has_up && end_index.is_some() {
        let current_number = get_number(line, start_index.unwrap(), end_index.unwrap() + 1);
        vec.push(current_number);
        println!("{}", current_number);
    } else if start_index.is_some() && has_up {
        let current_number = get_number(line, start_index.unwrap(), j + 1);
        vec.push(current_number);
        println!("{}", current_number);
    } else if has_up && end_index.is_some() {
        let current_number = get_number(line, j, end_index.unwrap() + 1);
        vec.push(current_number);
        println!("{}", current_number);
    } else {
        if start_index.is_some() {
            let current_number = get_number(line, start_index.unwrap(), j);
            vec.push(current_number);
            println!("{}", current_number);
        }

        if has_up {
            let current_number = get_number(line, j, j + 1);
            vec.push(current_number);
            println!("{}", current_number);
        }

        if end_index.is_some() {
            let current_number = get_number(line, j + 1, end_index.unwrap() + 1);
            vec.push(current_number);
            println!("{}", current_number);
        }
    }

    vec
}

fn process_left(line: &Vec<char>, j: usize) -> Option<usize> {
    let mut start_index = j;
    let mut has_left = false;
    for x in (0..j).rev() {
        if !line.get(x).unwrap().is_ascii_digit() {
            break;
        }
        start_index = x;
        has_left = true;
    }

    if has_left {
        Some(start_index)
    } else {
        None
    }
}

fn process_right(line: &Vec<char>, j: usize) -> Option<usize> {
    let mut end_index = j;
    let mut has_right = false;
    for x in (j + 1)..line.len() {
        if !line.get(x).unwrap().is_ascii_digit() {
            break;
        }
        end_index = x;
        has_right = true
    }

    if has_right {
        Some(end_index)
    } else {
        None
    }
}

fn get_number(line: &mut Vec<char>, start_index: usize, end_index: usize) -> i32 {
    let mut number: String = "".to_owned();
    for x in start_index..end_index {
        number.push(*line.get(x).unwrap());
        line[x] = '.';
    }
    number.parse::<i32>().unwrap()
}
