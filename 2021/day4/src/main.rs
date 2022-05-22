use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader, SeekFrom};

fn part1(lines: &Vec<i64>) {}
fn part2(lines: &Vec<i64>) {}
struct BingoBoard {
    board: Vec<Vec<i32>>,
}

impl BingoBoard {
    fn as_string(&self) {
        println!("{{");
        for row in &self.board {
            println!("{:?}", row);
        }
        println!("}}");
    }
}
fn main() -> std::io::Result<()> {
    // Base FileIO
    // Read the file name from a file and prepare to read
    println!("Enter the name of the file");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let file = match File::open(&buffer.trim()) {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_line(&mut contents)?;
    buf_reader.read_line(&mut contents)?;

    // The first line of the file is for guesses
    let mut guesses: Vec<i32> = Vec::new();
    for i in contents.trim().split(",") {
        guesses.push(i.parse::<i32>().unwrap());
    }
    contents.clear();

    // Create the bingo boards
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    let mut temp_board: Vec<Vec<i32>> = Vec::new();
    for line in buf_reader.lines() {
        contents = line.unwrap();
        if contents.trim().len() == 0 {
            let temp = BingoBoard { board: temp_board };
            bingo_boards.push(temp);
            temp_board = Vec::new();
        } else {
            let mut row: Vec<i32> = Vec::new();
            for val in contents.split_whitespace() {
                row.push(val.parse::<i32>().unwrap());
            }
            temp_board.push(row);
        }
    }
    let temp = BingoBoard { board: temp_board };
    bingo_boards.push(temp);
    for board in bingo_boards {
        board.as_string();
    }
    Ok(())
}
// --------------------------------------------------------------------------

//part1(&lines);
//part2(&lines);
