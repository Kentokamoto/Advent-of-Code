use std::io::stdin;

use day6;

fn main() -> std::io::Result<()> {
    println!("Enter the name of the file");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    let lantern_fish = day6::new(&buffer);
    println!("{:?}", lantern_fish.unwrap());

    Ok(())
}
