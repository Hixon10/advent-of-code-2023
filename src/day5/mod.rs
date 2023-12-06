use std::fs::read_to_string;

struct SearchRange {
    start_index: i64,
    end_index: i64,
    diff: i64,
}

pub fn solve1() {
    let file_str = read_to_string("src/day5/input.txt").unwrap();

    let mut seeds: Vec<i64> = Vec::new();

    let mut seed_to_soil: Vec<SearchRange> = Vec::new();
    let mut soil_to_fertilizer: Vec<SearchRange> = Vec::new();
    let mut fertilizer_to_water: Vec<SearchRange> = Vec::new();
    let mut water_to_light: Vec<SearchRange> = Vec::new();
    let mut light_to_temperature: Vec<SearchRange> = Vec::new();
    let mut temperature_to_humidity: Vec<SearchRange> = Vec::new();
    let mut humidity_to_location: Vec<SearchRange> = Vec::new();

    let mut current_map: &mut Vec<SearchRange> = &mut seed_to_soil;

    for (index, line) in file_str.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        if index == 0 {
            let x: Vec<&str> = line.split("seeds: ").collect();
            seeds = x
                .get(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            continue;
        }

        if line.eq("seed-to-soil map:") {
            current_map = &mut seed_to_soil;
            continue;
        } else if line.eq("soil-to-fertilizer map:") {
            current_map = &mut soil_to_fertilizer;
            continue;
        } else if line.eq("fertilizer-to-water map:") {
            current_map = &mut fertilizer_to_water;
            continue;
        } else if line.eq("water-to-light map:") {
            current_map = &mut water_to_light;
            continue;
        } else if line.eq("light-to-temperature map:") {
            current_map = &mut light_to_temperature;
            continue;
        } else if line.eq("temperature-to-humidity map:") {
            current_map = &mut temperature_to_humidity;
            continue;
        } else if line.eq("humidity-to-location map:") {
            current_map = &mut humidity_to_location;
            continue;
        }

        let current_numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        current_map.push(SearchRange {
            start_index: current_numbers[1],
            end_index: current_numbers[1] + current_numbers[2],
            diff: current_numbers[0] - current_numbers[1],
        });
    }

    let mut min_result: i64 = i64::MAX;

    for seed in seeds {
        let soil: i64 = convert(seed, &seed_to_soil);
        let fertilizer: i64 = convert(soil, &soil_to_fertilizer);
        let water: i64 = convert(fertilizer, &fertilizer_to_water);
        let light: i64 = convert(water, &water_to_light);
        let temperature: i64 = convert(light, &light_to_temperature);
        let humidity: i64 = convert(temperature, &temperature_to_humidity);
        let location: i64 = convert(humidity, &humidity_to_location);

        min_result = min_result.min(location)
    }

    println!("{}", min_result);
}

pub fn solve2() {
    let file_str = read_to_string("src/day5/input.txt").unwrap();

    let mut seeds: Vec<i64> = Vec::new();

    let mut seed_to_soil: Vec<SearchRange> = Vec::new();
    let mut soil_to_fertilizer: Vec<SearchRange> = Vec::new();
    let mut fertilizer_to_water: Vec<SearchRange> = Vec::new();
    let mut water_to_light: Vec<SearchRange> = Vec::new();
    let mut light_to_temperature: Vec<SearchRange> = Vec::new();
    let mut temperature_to_humidity: Vec<SearchRange> = Vec::new();
    let mut humidity_to_location: Vec<SearchRange> = Vec::new();

    let mut current_map: &mut Vec<SearchRange> = &mut seed_to_soil;

    for (index, line) in file_str.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        if index == 0 {
            let x: Vec<&str> = line.split("seeds: ").collect();
            seeds = x
                .get(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            continue;
        }

        if line.eq("seed-to-soil map:") {
            current_map = &mut seed_to_soil;
            continue;
        } else if line.eq("soil-to-fertilizer map:") {
            current_map = &mut soil_to_fertilizer;
            continue;
        } else if line.eq("fertilizer-to-water map:") {
            current_map = &mut fertilizer_to_water;
            continue;
        } else if line.eq("water-to-light map:") {
            current_map = &mut water_to_light;
            continue;
        } else if line.eq("light-to-temperature map:") {
            current_map = &mut light_to_temperature;
            continue;
        } else if line.eq("temperature-to-humidity map:") {
            current_map = &mut temperature_to_humidity;
            continue;
        } else if line.eq("humidity-to-location map:") {
            current_map = &mut humidity_to_location;
            continue;
        }

        let current_numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        current_map.push(SearchRange {
            start_index: current_numbers[1],
            end_index: current_numbers[1] + current_numbers[2],
            diff: current_numbers[0] - current_numbers[1],
        });
    }

    let mut min_result: i64 = i64::MAX;

    for x in (0..seeds.len()).step_by(2) {
        println!("seeds[x]={}", seeds[x]);
        for seed in seeds[x]..seeds[x] + seeds[x + 1] + 1 {
            let soil: i64 = convert(seed, &seed_to_soil);
            let fertilizer: i64 = convert(soil, &soil_to_fertilizer);
            let water: i64 = convert(fertilizer, &fertilizer_to_water);
            let light: i64 = convert(water, &water_to_light);
            let temperature: i64 = convert(light, &light_to_temperature);
            let humidity: i64 = convert(temperature, &temperature_to_humidity);
            let location: i64 = convert(humidity, &humidity_to_location);

            min_result = min_result.min(location)
        }
    }

    println!("{}", min_result);
}

fn convert(element: i64, map: &Vec<SearchRange>) -> i64 {
    for range in map {
        if element >= range.start_index && element <= range.end_index {
            return element + range.diff;
        }
    }

    element
}
