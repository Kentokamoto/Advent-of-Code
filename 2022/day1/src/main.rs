use day1::Elf;
use std::{env, fs, io, result::Result};

fn read_input() -> Result<String, Box<dyn std::error::Error>> {
    let path = env::current_dir();

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

fn get_elf_list(input: &String) -> Result<Vec<Elf>, Box<dyn std::error::Error>> {
    let mut vec = Vec::new();
    let mut elf = Elf::new();
    for line in input.lines() {
        if line.is_empty() {
            vec.push(elf);
            elf = Elf::new();
        } else {
            let val = line.parse::<i32>().unwrap();
            elf.add(val);
        }
    }
    vec.push(elf);
    Ok(vec)
}
fn part1(input: &Vec<Elf>) -> i32 {
    let mut most_calories = 0;
    for elf in input {
        if elf.get_total() > most_calories {
            most_calories = elf.get_total()
        }
    }
    most_calories
}
fn main() {
    let contents = read_input();
    let elf_list = get_elf_list(&contents.unwrap());
    let result1 = part1(&elf_list.unwrap());

    println!("Part 1: {}", result1);
}
