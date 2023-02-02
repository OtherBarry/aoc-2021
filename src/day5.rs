use std::collections::HashMap;
use std::fs;

fn parse_input(path: &str) -> Vec<Vec<Vec<i32>>> {
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(
            line.split(" -> ")
                .map(|x| {
                    x.split(",")
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(),
        );
    }
    return result;
}

fn get_points_on_line(start: Vec<i32>, end: Vec<i32>, allow_diagonal: bool) -> Vec<(i32, i32)> {
    // TODO: Improve with range
    let start_x = start[0];
    let start_y = start[1];
    let end_x = end[0];
    let end_y = end[1];
    let mut result = Vec::new();
    if start_x == end_x {
        let mut y = start_y;
        result.push((start_x, y));
        while y != end_y {
            if start_y < end_y {
                y += 1;
            } else {
                y -= 1;
            }
            result.push((start_x, y));
        }
    } else if start_y == end_y {
        let mut x = start_x;
        result.push((x, start_y));
        while x != end_x {
            if start_x < end_x {
                x += 1;
            } else {
                x -= 1;
            }
            result.push((x, start_y));
        }
    } else if allow_diagonal {
        let mut x = start_x;
        let mut y = start_y;
        result.push((x, y));
        while x != end_x && y != end_y {
            if start_x < end_x {
                x += 1;
            } else {
                x -= 1;
            }
            if start_y < end_y {
                y += 1;
            } else {
                y -= 1;
            }
            result.push((x, y));
        }
    }
    return result;
}

fn insert_or_add(map: &mut HashMap<(i32, i32), i32>, key: (i32, i32)) {
    match map.get(&key) {
        Some(x) => map.insert(key, x + 1),
        None => map.insert(key, 1),
    };
}

fn calculate_intersections(input: &Vec<Vec<Vec<i32>>>, allow_diagonal: bool) -> i32 {
    let mut covered_points = HashMap::<(i32, i32), i32>::new();
    for line in input.clone() {
        let points = get_points_on_line(line[0].clone(), line[1].clone(), allow_diagonal);
        for point in points {
            insert_or_add(&mut covered_points, point);
        }
    }
    let mut count = 0;
    for value in covered_points.values() {
        if value > &1 {
            count += 1;
        }
    }
    return count;
}

pub fn run(path: &str) {
    println!("Day 5:");
    let input = parse_input(path);
    println!("\tPart 1: {}", calculate_intersections(&input, false));
    println!("\tPart 2: {}", calculate_intersections(&input, true));
}
