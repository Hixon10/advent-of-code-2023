use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day1/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;

            for line in res.lines() {
                let mut first: char = '\0';
                let mut second: char = '\0';
                let mut first_set = false;
                let mut second_set = false;

                for ch in line.chars() {
                    if ch.is_ascii_digit() {
                        if !first_set {
                            first_set = true;
                            first = ch;
                        } else {
                            second_set = true;
                            second = ch;
                        }
                    }
                }

                if !first_set {
                    panic_any("!first_set")
                }

                let s2: String = if second_set {
                    format!("{}{}", first, second)
                } else {
                    format!("{}{}", first, first)
                };

                result += s2.parse::<i32>().unwrap();
            }

            println!("{}", result);
        }
        Err(_) => panic_any("file not found"),
    }
}

pub fn solve2() {
    let result = read_to_string("src/day1/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;

            for line in res.lines() {
                let mut first: char = '\0';
                let mut second: char = '\0';
                let mut first_set = false;
                let mut second_set = false;

                for (i, ch) in line.chars().enumerate() {
                    let d: Option<char> = if ch.is_ascii_digit() {
                        Some(ch)
                    } else if line.get(i..i + 3) == Some("one") {
                        Some('1')
                    } else if line.get(i..i + 3) == Some("two") {
                        Some('2')
                    } else if line.get(i..i + 5) == Some("three") {
                        Some('3')
                    } else if line.get(i..i + 4) == Some("four") {
                        Some('4')
                    } else if line.get(i..i + 4) == Some("five") {
                        Some('5')
                    } else if line.get(i..i + 3) == Some("six") {
                        Some('6')
                    } else if line.get(i..i + 5) == Some("seven") {
                        Some('7')
                    } else if line.get(i..i + 5) == Some("eight") {
                        Some('8')
                    } else if line.get(i..i + 4) == Some("nine") {
                        Some('9')
                    } else {
                        None
                    };

                    match d {
                        None => {}
                        Some(n) => {
                            if !first_set {
                                first_set = true;
                                first = n;
                            } else {
                                second_set = true;
                                second = n;
                            }
                        }
                    }
                }

                if !first_set {
                    panic_any("!first_set")
                }

                let s2: String = if second_set {
                    format!("{}{}", first, second)
                } else {
                    format!("{}{}", first, first)
                };

                result += s2.parse::<i32>().unwrap();
            }

            println!("{}", result);
        }
        Err(_) => panic_any("file not found"),
    }
}
