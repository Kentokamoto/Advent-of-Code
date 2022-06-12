use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Debug)]
pub struct LineSegment {
    pub start: Point,
    // Inclusive of End
    pub end: Point,
}

impl LineSegment {
    pub fn new(input: &String) -> Result<LineSegment, &str> {
        if input.is_empty() {
            return Err("Input is empty");
        }
        let valid_input = input.clone();
        let points: Vec<&str> = valid_input.split("->").collect();
        if points.len() != 2 {
            return Err("Input did not in format 'x1,y1 -> x2, y2'");
        }
        let point1: Vec<&str> = points[0].trim().split(",").collect();
        let point2: Vec<&str> = points[1].trim().split(",").collect();
        let p1 = Point {
            x: point1[0].parse::<i32>().unwrap(),
            y: point1[1].parse::<i32>().unwrap(),
        };
        let p2 = Point {
            x: point2[0].parse::<i32>().unwrap(),
            y: point2[1].parse::<i32>().unwrap(),
        };
        let output = LineSegment { start: p1, end: p2 };
        Ok(output)
    }
    pub fn is_diagonal(&self) -> bool {
        (self.start.x != self.end.x) && (self.start.y != self.end.y)
    }
    pub fn is_45_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }
    pub fn is_vertical(&self) -> bool {
        (self.start.x == self.end.x) && (self.start.y != self.end.y)
    }
    pub fn is_horizontal(&self) -> bool {
        (self.start.x != self.end.x) && (self.start.y == self.end.y)
    }
}
pub fn new(file_name: &String) -> Result<Vec<LineSegment>, &str> {
    let file = match File::open(&file_name.trim()) {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let buf_reader = BufReader::new(file);

    let mut output = Vec::new();
    for line in buf_reader.lines() {
        output.push(LineSegment::new(&line.unwrap()).unwrap());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_line_segment() {
        let input = String::from("3,4 -> 1,4");
        let ls = LineSegment::new(&input).unwrap();
        let answer = LineSegment {
            start: Point { x: 3, y: 4 },
            end: Point { x: 1, y: 4 },
        };
        assert_eq!(answer, ls);
    }
    #[test]
    fn check_diagonals() {
        let test1 = LineSegment {
            start: Point { x: 0, y: 4 },
            end: Point { x: 4, y: 0 },
        };
        assert_eq!(true, test1.is_diagonal());
        let test2 = LineSegment {
            start: Point { x: 3, y: 4 },
            end: Point { x: 1, y: 4 },
        };
        assert_eq!(false, test2.is_diagonal());
        let test3 = LineSegment {
            start: Point { x: 0, y: 8 },
            end: Point { x: 8, y: 0 },
        };
        assert_eq!(true, test3.is_45_diagonal());
        let test4 = LineSegment {
            start: Point { x: 1, y: 8 },
            end: Point { x: 8, y: 0 },
        };
        assert_eq!(false, test4.is_45_diagonal());
    }
    #[test]
    fn check_vertical() {
        let test1 = LineSegment {
            start: Point { x: 0, y: 4 },
            end: Point { x: 0, y: 9 },
        };
        assert_eq!(true, test1.is_vertical());
    }
    #[test]
    fn check_horizontal() {
        let test1 = LineSegment {
            start: Point { x: 0, y: 4 },
            end: Point { x: 9, y: 4 },
        };
        assert_eq!(true, test1.is_horizontal());
    }
}
