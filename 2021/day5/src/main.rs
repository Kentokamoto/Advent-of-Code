use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::stdin;

use day5::{LineSegment, Point};

fn part1(segments: &Vec<LineSegment>) -> Result<(), &str> {
    let part1_segments = segments.clone();
    let mut vents: HashMap<Point, i32> = HashMap::new();
    for segment in part1_segments {
        if segment.is_diagonal() {
            continue;
        }
        if segment.is_vertical() {
            let max = max(segment.start.y, segment.end.y);
            let min = min(segment.start.y, segment.end.y);
            for y in min..max + 1 {
                let tmp_point = Point {
                    x: segment.start.x,
                    y: y,
                };
                let count = vents.entry(tmp_point).or_insert(0);
                *count += 1;
            }
        }
        if segment.is_horizontal() {
            let max = max(segment.start.x, segment.end.x);
            let min = min(segment.start.x, segment.end.x);
            for x in min..max + 1 {
                let tmp_point = Point {
                    x: x,
                    y: segment.start.y,
                };
                let count = vents.entry(tmp_point).or_insert(0);
                *count += 1;
            }
        }
    }
    let mut counter = 0;
    for vent in vents {
        if vent.1 >= 2 {
            counter += 1;
        }
    }
    println!("Points: {}", counter);
    Ok(())
}

fn part2(segments: &Vec<LineSegment>) -> Result<(), &str> {
    let part2_segments = segments.clone();
    let mut vents: HashMap<Point, i32> = HashMap::new();
    for segment in part2_segments {
        if segment.is_45_diagonal() {
            let count = (segment.start.x - segment.end.x).abs();
            let polarity_y = if segment.start.y > segment.end.y {-1} else {1}; 
            let polarity_x = if segment.start.x > segment.end.x {-1} else {1}; 
            for i in 0..count+1 {
                let tmp_point = Point {
                    x: segment.start.x + (polarity_x* i),
                    y: segment.start.y + (polarity_y* i),
                };
                let count = vents.entry(tmp_point).or_insert(0);
                *count += 1;
            } 
        } else if segment.is_vertical() {
            let max = max(segment.start.y, segment.end.y);
            let min = min(segment.start.y, segment.end.y);
            for y in min..max + 1 {
                let tmp_point = Point {
                    x: segment.start.x,
                    y: y,
                };
                let count = vents.entry(tmp_point).or_insert(0);
                *count += 1;
            }
        } else if segment.is_horizontal() {
            let max = max(segment.start.x, segment.end.x);
            let min = min(segment.start.x, segment.end.x);
            for x in min..max + 1 {
                let tmp_point = Point {
                    x: x,
                    y: segment.start.y,
                };
                let count = vents.entry(tmp_point).or_insert(0);
                *count += 1;
            }
        }
    }
    let mut counter = 0;
    for vent in vents {
        if vent.1 >= 2 {
            counter += 1;
        }
    }

    println!("Points: {}", counter);
    Ok(())
}
fn main() -> std::io::Result<()> {
    println!("Enter the name of the file");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    let line_segments = day5::new(&buffer).unwrap();
    part1(&line_segments).unwrap();
    part2(&line_segments).unwrap();
    Ok(())
}
