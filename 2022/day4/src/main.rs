use day4::CleanupGroup;
use std::{fs, io, result::Result};

fn read_input() -> Result<String, Box<dyn std::error::Error>> {
    println!("Enter a filename: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}"),
    }
    println!("Opening filename: {}", &input);
    let contents: String = match fs::read_to_string(&input.trim()) {
        Ok(buffer) => buffer,
        Err(e) => {
            println!("Error reading file: {e}");
            return Err(Box::new(e));
        }
    };

    return Ok(contents);
}
fn parse_group(raw: &String) -> CleanupGroup {
    let min_max: Vec<&str> = raw.split("-").collect();
    CleanupGroup::new(
        min_max[0].parse::<i32>().unwrap(),
        min_max[1].parse::<i32>().unwrap(),
    )
    .unwrap()
}
fn parse_input(contents: &String) -> Vec<(CleanupGroup, CleanupGroup)> {
    let mut group_pairs = Vec::new();
    for line in contents.lines() {
        let split_group: Vec<&str> = line.split(",").collect();
        let group1 = parse_group(&split_group[0].to_string());
        let group2 = parse_group(&split_group[1].to_string());
        group_pairs.push((group1, group2));
    }
    group_pairs
}
fn part1(pairs: &Vec<(CleanupGroup, CleanupGroup)>) -> i32 {
    let mut total = 0;
    for group in pairs {
        if group.0.fully_contains(&group.1) || group.1.fully_contains(&group.0) {
            total += 1;
        }
    }
    total
}
fn part2(pairs: &Vec<(CleanupGroup, CleanupGroup)>) -> i32 {
    let mut total = 0;
    for group in pairs {
        if group.0.overlaps(&group.1) || group.1.overlaps(&group.0) {
            total += 1;
        }
    }
    total
}
fn main() {
    let contents = read_input();
    let group_pairs = parse_input(&contents.as_ref().unwrap());
    let result1 = part1(&group_pairs);
    let result2 = part2(&group_pairs);
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}
