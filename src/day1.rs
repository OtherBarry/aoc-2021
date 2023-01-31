use std::fs;

fn parse_input(path: &str) -> Vec<i32> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.parse::<i32>().unwrap());
    }
    return result;
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut last = i32::MAX;
    let mut count = 0;
    for current in input {
        if last < *current {
            count += 1;
        }
        last = *current;
    }
    return count;
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut last = i32::MAX;
    let mut count = 0;
    for i in 3..input.len() + 1 {
        let window_size = input[i - 3..i].iter().sum();
        if window_size > last {
            count += 1;
        }
        last = window_size;
    }
    return count;
}

pub fn run(path: &str) {
    println!("Day 1:");
    let input = parse_input(path);
    println!("\tPart 1: {}", part1(&input));
    println!("\tPart 2: {}", part2(&input));
}
