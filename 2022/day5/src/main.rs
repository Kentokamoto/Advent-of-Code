use day5::CargoCrane;
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
fn part1(contents: &String) -> String {
    let mut setup: String = String::new();
    let mut lines = contents.lines();
    let mut curr_line = lines.next();
    while !curr_line.unwrap().is_empty() {
        setup.push_str(curr_line.unwrap());
        setup.push_str("\n");
        curr_line = lines.next();
    }
    let mut cargo_crane = CargoCrane::new();
    cargo_crane.parse_input(&setup);
    println!("{:?}", cargo_crane);
    curr_line = lines.next();
    while curr_line != None {
        let mut sp = curr_line.unwrap().split_whitespace();
        let move_count = sp.nth(1).unwrap().parse::<i32>().unwrap();
        let from_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        cargo_crane.move_crate(&move_count, &from_crate, &to_crate);
        curr_line = lines.next();
    }
    cargo_crane.get_top()
}
fn part2(contents: &String) -> String {
    let mut setup: String = String::new();
    let mut lines = contents.lines();
    let mut curr_line = lines.next();
    while !curr_line.unwrap().is_empty() {
        setup.push_str(curr_line.unwrap());
        setup.push_str("\n");
        curr_line = lines.next();
    }
    let mut cargo_crane = CargoCrane::new();
    cargo_crane.parse_input(&setup);
    println!("{:?}", cargo_crane);
    curr_line = lines.next();
    while curr_line != None {
        let mut sp = curr_line.unwrap().split_whitespace();
        let move_count = sp.nth(1).unwrap().parse::<i32>().unwrap();
        let from_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        cargo_crane.move_crate_2(&move_count, &from_crate, &to_crate);
        curr_line = lines.next();
    }
    cargo_crane.get_top()
}
fn main() {
    let contents = read_input();
    let result1 = part1(&contents.as_ref().unwrap());
    println!("Result 1: {}", result1);
    let result2 = part2(&contents.as_ref().unwrap());
    println!("Result 2: {}", result2);
}
