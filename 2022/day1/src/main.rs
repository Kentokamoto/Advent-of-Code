use day1::Elf;
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

fn part2(input: &mut Vec<Elf>) -> i32 {
    input.sort_by(|a, b| a.get_total().cmp(&b.get_total()).reverse());
    let thing = &input[0..3];
    for t in thing {
        println!("{}", t.get_total());
    }
    thing.iter().map(|x| x.get_total()).sum()
}
fn main() {
    let contents = read_input();
    let mut elf_list = get_elf_list(&contents.unwrap()).unwrap();
    let result1 = part1(&elf_list);
    let result2 = part2(&mut elf_list);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
