use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point{
    x: i32,
    y: i32
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct LineSegment{
    start: Point,
    // Inclusive of End
    end: Point
}
impl LineSegment{
    pub fn new(input: &String) -> Result<LineSegment, &str>{
        if input.is_empty(){
            return Err("Input is empty");
        }
        println!("Input: {}", input);
        let valid_input = input.clone();
        let points: Vec<&str>= valid_input.split("->").collect();
        if points.len() != 2{
            return Err("Input did not in format 'x1,y1 -> x2, y2'")
        }
        println!("Split: {:?}", points);
        let point1:Vec<&str> = points[0].trim().split(",").collect();
        let point2:Vec<&str> = points[1].trim().split(",").collect();
        let p1 = Point{
            x: point1[0].parse::<i32>().unwrap(),
            y: point1[1].parse::<i32>().unwrap()
        };
        let p2 = Point{
            x: point2[0].parse::<i32>().unwrap(),
            y: point2[1].parse::<i32>().unwrap()
        };
        let output = LineSegment{
            start: p1,
            end: p2
        };
        Ok(output)
    }
    pub fn isDiagonal(self) -> bool {
        (self.start.x == self.end.x )||  ( self.start.y == self.end.y)
    }
}
pub fn new(file_name: &String) -> Result<Vec<LineSegment>, &str>{
    let file = match File::open(&file_name.trim()) {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file: {:?}", error), };
    let buf_reader = BufReader::new(file);

    let mut output = Vec::new();
    for line in buf_reader.lines(){
        output.push(LineSegment::new(&line.unwrap()).unwrap());
    }
    Ok(output)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_line_segment(){
        let input = String::from("3,4 -> 1,4");
        let ls = LineSegment::new(&input).unwrap();
        let answer = LineSegment {
            start: Point {
                x: 3,
                y: 4
            },
            end: Point{
                x: 1,
                y: 4
            }
        };
        assert_eq!(answer, ls);
    }
    #[test]
    fn check_diagonals(){
        let test1 = LineSegment {
            start: Point {
                x: 0,
                y: 4
            },
            end: Point{
                x: 4,
                y: 0
            }
        };
        assert_eq!(false, test1.isDiagonal());
        let test2 = LineSegment {
            start: Point {
                x: 3,
                y: 4
            },
            end: Point{
                x: 1,
                y: 4
            }
        };
        assert_eq!(true, test2.isDiagonal());
    }
}

