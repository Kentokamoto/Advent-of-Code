use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};

fn part1(guesses: &Vec<String>, bingo_boards: &Vec<BingoBoard>) {
    println!("Part 1");
    let mut board_copy = bingo_boards.clone();
    for guess in guesses {
        for b in &mut board_copy {
            let mut row = 0;
            let mut col = 0;
            if b.has_value(guess, &mut row, &mut col) {
                b.mark_board(&row, &col);
                if b.is_bingo(&row, &col) {
                    println!("Found Bingo: \n {}", b);
                    println!(
                        "final Score: {}",
                        b.sum_unmarked() * guess.parse::<i32>().unwrap()
                    );
                    return;
                }
            }
        }
    }
}
fn part2(guesses: &Vec<String>, bingo_boards: &Vec<BingoBoard>) {
    println!("Part 2");
    let mut board_copy = bingo_boards.clone();
    let mut completed_board: HashSet<usize> = HashSet::new();
    for guess in guesses {
        for pos in 0..board_copy.len() {
            if completed_board.contains(&pos) {
                continue;
            }
            let mut row = 0;
            let mut col = 0;
            if board_copy[pos].has_value(guess, &mut row, &mut col) {
                board_copy[pos].mark_board(&row, &col);
                if board_copy[pos].is_bingo(&row, &col) {
                    completed_board.insert(pos);
                    if completed_board.len() == bingo_boards.len() {
                        println!("Found lastBingo: \n {}", board_copy[pos]);
                        println!(
                            "final Score: {}",
                            board_copy[pos].sum_unmarked() * guess.parse::<i32>().unwrap()
                        );
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct BingoBoard {
    board: Vec<Vec<String>>,
}
impl BingoBoard {
    fn has_value(&self, val: &String, r: &mut usize, c: &mut usize) -> bool {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                if self.board[row][col].eq(val) {
                    *r = row;
                    *c = col;
                    return true;
                }
            }
        }
        false
    }
    fn mark_board(&mut self, row: &usize, col: &usize) {
        if self.board[*row][*col].starts_with('x') {
            return;
        }
        self.board[*row][*col] = String::from("x") + &self.board[*row][*col];
    }
    fn is_bingo(&self, row: &usize, col: &usize) -> bool {
        // Check Row:
        let mut bingo_in_row = true;
        for col in &self.board[*row] {
            if !col.starts_with("x") {
                bingo_in_row = false;
                break;
            }
        }
        if bingo_in_row {
            return true;
        }
        // Check Col:
        let mut bingo_in_col = true;
        for r in 0..self.board.len() {
            if !self.board[r][*col].starts_with("x") {
                bingo_in_col = false;
                break;
            }
        }
        if bingo_in_col {
            return true;
        }
        false
    }
    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            for col in row {
                let val = col.parse::<i32>();
                sum = match val {
                    Ok(val) => sum + val,
                    Err(_) => sum,
                }
            }
        }
        sum
    }
}
impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        for row in &self.board {
            write!(f, "{:?}\n", row)?;
        }
        write!(f, "}}")?;
        Ok(())
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
    let mut guesses: Vec<String> = Vec::new();
    for i in contents.trim().split(",") {
        guesses.push(i.to_string());
    }
    contents.clear();

    // Create the bingo boards
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    let mut temp_board: Vec<Vec<String>> = Vec::new();
    for line in buf_reader.lines() {
        contents = line.unwrap();
        if contents.trim().len() == 0 {
            let temp = BingoBoard { board: temp_board };
            bingo_boards.push(temp);
            temp_board = Vec::new();
        } else {
            let mut row: Vec<String> = Vec::new();
            for val in contents.split_whitespace() {
                row.push(val.to_string());
            }
            temp_board.push(row);
        }
    }
    let temp = BingoBoard { board: temp_board };
    bingo_boards.push(temp);

    part1(&mut guesses, &mut bingo_boards);
    println!("");
    part2(&mut guesses, &mut bingo_boards);

    Ok(())
}
// --------------------------------------------------------------------------

//part1(&lines);
//part2(&lines);
