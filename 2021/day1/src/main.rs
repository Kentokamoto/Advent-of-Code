use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1(depths: &Vec<i64>){
    println!("Part 1");
    let mut prev = depths[0];
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

fn part2(depths: &Vec<i64>){
    println!("Part 2");
    let mut prev_sum = depths[0] + depths[1] + depths[2];
    let mut count : u16 = 0;
    for index in 1..(depths.len()-2){
        let curr_sum = depths[index] + depths[index+1]+ depths[index+2];
        if curr_sum > prev_sum {
            count += 1;
        }
        prev_sum = curr_sum;
    }
    println!("Part 2 Answer: {}", count);
    println!("End of Part 2");
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
    part1(&lines);
    part2(&lines);
}
