use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1(depths: Vec<i64>){
    println!("Part 1");
    let mut prev = depths[0];
    println!("First number: {}", prev);
    let mut count : u16 = 0;

    for index in 1..depths.len(){
        let curr = depths[index];
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }

    println!("Part 1 Answer: {}", count);
    println!("End of Part1");
}


fn main() {
    println!("Opening and reading file");
    let filename = "input1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<i64> = reader.lines()
                                .map(|line| line.unwrap().parse::<i64>().unwrap())
                                .collect();

    println!("Number of lines: {}\n", lines.len());
    part1(lines);
}
