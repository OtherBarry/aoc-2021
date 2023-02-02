use std::fs;
use cached::proc_macro::cached;

fn parse_input(path: &str) -> Vec<i64> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    return input.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
}

#[cached]
fn calculate_spawns_in_period(starting_value: i64, period: i64) -> i64 {
    let mut result = 1;
    let remaining_period = period - starting_value;
    if remaining_period > 0 {
        for i in (0..remaining_period).rev().step_by(7) {
            result += calculate_spawns_in_period(8, i);
        }
    }
    return result;
}

fn simulate_lanternfish_recursive(input: &Vec<i64>, days: usize) -> i64 {
    let mut count = 0;
    for start in input {
        count += calculate_spawns_in_period(*start, days as i64);
    }
    return count;
}

fn part2() -> i64 {
    return 0;
}

pub fn run(path: &str) {
    println!("Day 6:");
    let mut input = parse_input(path);
    println!("\tPart 1: {}", simulate_lanternfish_recursive(&input, 80));
    println!("\tPart 2: {}", simulate_lanternfish_recursive(&mut input, 256));
}
