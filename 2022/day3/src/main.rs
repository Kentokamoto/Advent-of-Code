use day3::{ElfGroup, Rucksack};
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

fn parse_input(contents: &String) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in contents.lines() {
        let compartment_size = line.len() / 2;
        let mut rucksack = Rucksack::new();
        let first_half = &line[0..compartment_size];
        let second_half = &line[compartment_size..];
        for item_index in 0..compartment_size {
            let item = first_half.chars().nth(item_index).unwrap();
            rucksack.add_to_compartment1(item);
            let item = second_half.chars().nth(item_index).unwrap();
            rucksack.add_to_compartment2(item);
        }
        rucksacks.push(rucksack);
    }
    rucksacks
}

fn parse_elf_group(contents: &String) -> Vec<ElfGroup> {
    let mut elf_groups = Vec::new();
    let mut lines = contents.lines();
    let mut line = lines.next();
    while line != None {
        let mut elf_group = ElfGroup::new();
        for rucksack_num in 1..4 {
            for c in line.unwrap().chars() {
                elf_group.add_to_rucksack(&rucksack_num, c).unwrap();
            }
            line = lines.next();
        }
        elf_groups.push(elf_group);
    }
    elf_groups
}
fn part1(rucksacks: &Vec<Rucksack>) -> i32 {
    let mut total = 0;
    for rucksack in rucksacks {
        let c = rucksack.find_duplicate().unwrap();
        if c.is_ascii_uppercase() {
            // @ is the character before A in ascii
            total += (c as i32) - ('@' as i32) + 26;
        } else {
            // ` is the character before a in ascii
            total += (c as i32) - ('`' as i32);
        }
    }
    total
}

fn part2(elfgroups: &Vec<ElfGroup>) -> i32 {
    let mut total = 0;
    for elf_group in elfgroups {
        let c = elf_group.find_badge().unwrap();
        if c.is_ascii_uppercase() {
            // @ is the character before A in ascii
            total += (c as i32) - ('@' as i32) + 26;
        } else {
            // ` is the character before a in ascii
            total += (c as i32) - ('`' as i32);
        }
    }
    total
}

fn main() {
    let contents = read_input();
    let rucksacks = parse_input(&contents.as_ref().unwrap());
    let result1 = part1(&rucksacks);
    let elf_groups = parse_elf_group(&contents.unwrap());
    let result2 = part2(&elf_groups);
    println!("Result1: {}", result1);
    println!("Result2: {}", result2);
}
