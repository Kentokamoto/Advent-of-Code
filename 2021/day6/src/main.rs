use std::io::stdin;
use std::thread::sleep;

use day6::{self, LanternFish};


fn part1(lantern_fish: &mut Vec<LanternFish>){
    let final_days = 80;
    let mut lantern_fish_cp = lantern_fish.clone();
    for _ in 0..final_days {
        let mut temp: Vec<LanternFish> = Vec::new();
        for fish in lantern_fish_cp.iter_mut(){
            if fish.time_to_spawn() {
                fish.reset();
                let new_timer = 6;
                let start_timer = 8;
                temp.push(LanternFish::new(start_timer, new_timer));
            }else {
                fish.decrement();
            }
        }
        lantern_fish_cp.append(&mut temp);
        //println!("{:?}\n", lantern_fish_cp);
    }

    println!("Number of Fish after 80 days: {}", lantern_fish_cp.len());
}

fn main() -> std::io::Result<()> {
    println!("Enter the name of the file");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    let mut lantern_fish = day6::new(&buffer).unwrap();
    println!("{:?}", lantern_fish);
    part1(&mut lantern_fish);
    Ok(())
}
