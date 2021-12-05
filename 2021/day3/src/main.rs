use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1(lines: &Vec<String>){
    println!("Part 1");
    let mut tally: Vec<i32> = vec![0; lines[0].len()];
    println!("Tally size {:?}", tally );
    for line in lines{
        for i in 0..line.chars().count() {
            if line.chars().nth(i).unwrap() == '1'{
                tally[i] += 1; 
            }else{
                tally[i] -= 1; 
            }
        }
    }
    println!("Tally {:?}", tally );
    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;
    for t in tally{
        if t > 0{
            println!("Greater");
            gamma |= 1;
            epsilon |= 0;
            gamma <<= 1;
            epsilon <<= 1;
        }else{
            println!("Lesser");
            gamma |= 0;
            epsilon |= 1;
            gamma <<= 1;
            epsilon <<= 1;
        }
    }
    gamma >>= 1;
    epsilon >>= 1;
    println!("Gamma {}", gamma );
    println!("Epsilon {}", epsilon );
    println!("Product {}", gamma*epsilon );
    println!("End of Part1");
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
}
