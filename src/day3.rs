use std::fs;

fn parse_input(path: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    return result;
}

fn part1(input: &Vec<Vec<u32>>) -> i32 {
    let mut count = [0; 12]; // TODO: get this from input
    for line in input {
        for (i, digit) in line.iter().enumerate() {
            count[i] += digit;
        }
    }
    let mut gamma_string = "".to_owned();
    let mut epsilon_string = "".to_owned();
    let half = input.len() as f64 / 2.0;

    for i in count.iter() {
        if *i as f64 > half {
            gamma_string += "1";
            epsilon_string += "0";
        }
        else {
            gamma_string += "0";
            epsilon_string += "1";
        }
    }

    let gamma = i32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_string, 2).unwrap();
    return gamma * epsilon;
}

fn part2(input: &Vec<Vec<u32>>) -> i32 {
    // TODO: Extract gamma and epsilon calculation into function
    //     Copy input into two filterable vectors
    //     Filter out values using gamma/epsilon as bit mask
    return 0;
}

pub fn run(path: &str) {
    println!("Day 3:");
    let input = parse_input(path);
    println!("\tPart 1: {}", part1(&input));
    println!("\tPart 2: {}", part2(&input));
}