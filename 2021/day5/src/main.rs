use std::io::stdin;

use day5::LineSegment;

fn main() -> std::io::Result<()>{
    println!("Enter the name of the file");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let line_segments = day5::new(&buffer); 
    println!("Line Segments: {:?}", line_segments);

    Ok(())
}
