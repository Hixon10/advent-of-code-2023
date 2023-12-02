use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day2/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;

            for line in res.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    panic_any("if (parts.len() != 2")
                }

                let game_index = parts
                    .first()
                    .unwrap()
                    .strip_prefix("Game ")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();

                let mut possible = true;

                for round in parts.get(1).unwrap().split(';') {
                    let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

                    for color_count in round.trim().split(", ") {
                        let cc: Vec<&str> = color_count.split(' ').collect();
                        let new_count = cc.first().unwrap().parse::<i32>().unwrap();
                        let color = cc.get(1).unwrap();

                        let old_count = colors.get(color).unwrap();
                        colors.insert(color, new_count + old_count);
                    }

                    if !(colors.get("red").unwrap() <= &12
                        && colors.get("green").unwrap() <= &13
                        && colors.get("blue").unwrap() <= &14)
                    {
                        possible = false;
                    }
                }

                if possible {
                    result += game_index;
                }
            }

            println!("{}", result);
        }
        Err(_) => panic_any("file not found"),
    }
}

pub fn solve2() {
    let result = read_to_string("src/day2/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;

            for line in res.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    panic_any("if (parts.len() != 2")
                }

                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                for round in parts.get(1).unwrap().split(';') {
                    let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

                    for color_count in round.trim().split(", ") {
                        let cc: Vec<&str> = color_count.split(' ').collect();
                        let new_count = cc.first().unwrap().parse::<i32>().unwrap();
                        let color = cc.get(1).unwrap();

                        let old_count = colors.get(color).unwrap();
                        colors.insert(color, new_count + old_count);
                    }

                    max_red = max(max_red, *colors.get("red").unwrap());
                    max_green = max(max_green, *colors.get("green").unwrap());
                    max_blue = max(max_blue, *colors.get("blue").unwrap());
                }

                result += max_red * max_green * max_blue
            }

            println!("{}", result);
        }
        Err(_) => panic_any("file not found"),
    }
}
