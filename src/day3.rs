use std::fs;

const LINE_LENGTH: usize = 12; // TODO: get this from input

fn parse_input(path: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }
    return result;
}

fn part1(input: &Vec<Vec<u32>>) -> i32 {
    let mut count = [0; LINE_LENGTH];
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
        } else {
            gamma_string += "0";
            epsilon_string += "1";
        }
    }

    let gamma = i32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_string, 2).unwrap();
    return gamma * epsilon;
}

fn part2(input: &Vec<Vec<u32>>) -> i32 {
    let mut oxygen_generator_vec = input.clone();
    for i in 0..LINE_LENGTH {
        let (a, b): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
            oxygen_generator_vec.into_iter().partition(|x| x[i] == 0);
        if a.len() > b.len() {
            oxygen_generator_vec = a;
        } else {
            oxygen_generator_vec = b;
        }
        if oxygen_generator_vec.len() == 1 {
            break;
        }
    }
    println!("Oxygen Generator: {:?}", oxygen_generator_vec);
    let oxygen_generator_string = oxygen_generator_vec[0]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let oxygen_generator = i32::from_str_radix(&oxygen_generator_string, 2).unwrap();

    let mut c02_scrubber_vec = input.clone();
    for i in 0..LINE_LENGTH {
        let (a, b): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
            c02_scrubber_vec.into_iter().partition(|x| x[i] == 0);
        if a.len() <= b.len() {
            c02_scrubber_vec = a;
        } else {
            c02_scrubber_vec = b;
        }
        if c02_scrubber_vec.len() == 1 {
            break;
        }
    }
    println!("CO2 Scrubber: {:?}", c02_scrubber_vec);
    let c02_scrubber_string = c02_scrubber_vec[0]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let c02_scrubber = i32::from_str_radix(&c02_scrubber_string, 2).unwrap();

    return oxygen_generator * c02_scrubber;
}

pub fn run(path: &str) {
    println!("Day 3:");
    let input = parse_input(path);
    println!("\tPart 1: {}", part1(&input));
    println!("\tPart 2: {}", part2(&input));
}
