use day2::*;
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

fn parse_input(input: &String) -> Result<Vec<Strategy>, Box<dyn std::error::Error>> {
    let mut vec: Vec<Strategy> = Vec::new();
    for line in input.lines() {
        let moves: Vec<&str> = line.split_whitespace().collect();
        let opponent = moves[0];
        let me = moves[1];
        vec.push(Strategy::new(opponent, me));
    }
    Ok(vec)
}

fn part1(input: &Vec<Strategy>) -> i32 {
    input.iter().map(|x| x.result()).sum()
}
fn main() {
    let contents = read_input();
    let strat = parse_input(&contents.unwrap());
    let result1 = part1(&strat.unwrap());
    println!("Result 1: {}", result1);
}
