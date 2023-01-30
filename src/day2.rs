use std::fs;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn parse_input(path: &str) -> Vec<Direction> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut result = Vec::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let value = split.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => result.push(Direction::Forward(value)),
            "down" => result.push(Direction::Down(value)),
            "up" => result.push(Direction::Up(value)),
            _ => panic!("Unknown direction")
        }
    }
    return result;
}

fn part1(input: &Vec<Direction>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for direction in input {
        match direction {
            Direction::Forward(value) => horizontal += value,
            Direction::Down(value) => depth += value,
            Direction::Up(value) => depth -= value
        }
    }
    return horizontal * depth;
}

fn part2(input: &Vec<Direction>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for direction in input {
        match direction {
            Direction::Forward(value) => {
                horizontal += value;
                depth += aim * value;
            },
            Direction::Down(value) => aim += value,
            Direction::Up(value) => aim -= value
        }
    }
    return horizontal * depth;
}

pub fn run(path: &str) {
    println!("Day 2:");
    let input = parse_input(path);
    println!("\tPart 1: {}", part1(&input));
    println!("\tPart 2: {}", part2(&input));
}