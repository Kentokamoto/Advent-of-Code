use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1(lines: &Vec<String>){
    println!("Part 1");
    let mut vert : i64 = 0;
    let mut horiz : i64 = 0;

    for line in lines{
        let mut cmd = line.splitn(2, ' ');
        let direction = cmd.next().unwrap();
        let value = cmd.next()
                       .unwrap()
                       .parse::<i64>()
                       .unwrap();
        
        if direction == "down" {
            vert += value;
        }else if direction == "up" {
            vert -= value;
        }else if direction == "forward" {
            horiz += value;
        }
        //println!("{} with split at: {}, {}", line, direction, value);
    }
    let result = vert * horiz;
    println!("Result: {}", result);
    println!("End of Part1");
}

fn part2(lines: &Vec<String>){
    println!("Part 2");
    let mut aim : i64 = 0;
    let mut horiz : i64 = 0;
    let mut depth : i64 = 0;

    for line in lines{
        let mut cmd = line.splitn(2, ' ');
        let direction = cmd.next().unwrap();
        let value = cmd.next()
                       .unwrap()
                       .parse::<i64>()
                       .unwrap();
        
        if direction == "down" {
            aim += value;
        }else if direction == "up" {
            aim -= value;
        }else if direction == "forward" {
            horiz += value;
            depth += aim * value;
        }
        //println!("{} with split at: {}, {}", line, direction, value);
    }
    let result = depth * horiz;
    println!("Result: {}", result);
    println!("End of Part 2");
}

fn main() {
    println!("Opening and reading file");
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
                                .map(|line| line.unwrap().parse::<String>().unwrap())
                                .collect();

    println!("Number of lines: {}\n", lines.len());
    part1(&lines);
    part2(&lines);
}
