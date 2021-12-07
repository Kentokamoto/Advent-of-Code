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
            //println!("Greater");
            gamma |= 1;
            epsilon |= 0;
            gamma <<= 1;
            epsilon <<= 1;
        }else{
            //println!("Lesser");
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
fn part2(lines: &Vec<String>){
    println!("Part 2");
    let mut bit_index = 0;
    let mut oxygen : Vec<String> = lines.clone();
    let mut co2 : Vec<String> = lines.clone();
    while oxygen.len() > 1 || co2.len() > 1{
        let mut val = 0;
        let mut ones : Vec<String> = Vec::new();
        let mut zeros : Vec<String> = Vec::new(); 
        // Oxygen 
        if oxygen.len() > 1{
            for o in oxygen{
                if o.chars().nth(bit_index).unwrap() == '1'{
                    val += 1;
                    ones.push(o.clone());
                }else{
                    val -= 1;
                    zeros.push(o.clone());
                }
            }
            if val >= 0{
                // Keep the Ones
                oxygen = ones;
            }else{
                // Keep the Zeros
                oxygen = zeros;
            }
        }
        ones = Vec::new();
        zeros = Vec::new();
        val = 0;
        // CO2
        if co2.len() > 1{
            for c in co2{
                if c.chars().nth(bit_index).unwrap() == '1'{
                    val += 1;
                    ones.push(c.clone())
                }else{
                    val -= 1;
                    zeros.push(c.clone())
                }
            }
            if val >= 0{
                // Keep the Zeros
                co2 = zeros;
            }else{
                co2 = ones;
            }
        }
        println!("Oxygen Size:{}", oxygen.len());
        println!("CO2 Size:{}", co2.len());
        bit_index+= 1;
    }
    println!("Oxygen: {:?}", oxygen);
    println!("CO2: {:?}", co2);
    println!("Product: {:?}", isize::from_str_radix(&oxygen[0], 2).unwrap()*isize::from_str_radix(&co2[0], 2).unwrap());
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
