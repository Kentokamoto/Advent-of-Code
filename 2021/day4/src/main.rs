use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::fmt;

fn part1(guesses: &mut Vec<String>, bingo_boards: &Vec<BingoBoard>) {
    let mut board_copy = bingo_boards.clone();
    for guess in guesses{
        println!("Current Guess: {}", guess);
        for b in &mut board_copy{
            let mut row = 0;
            let mut col = 0;
            if b.has_value(guess, &mut row, &mut col) {
                println!("Found at {}, {} on board: \n{}", row, col, b);
                b.mark_board(&row, &col);
            }
        }
    }

}
fn part2(lines: &Vec<i64>) {}

#[derive(Clone)]
struct BingoBoard {
    board: Vec<Vec<String>>,
}
impl BingoBoard {
    fn has_value(&self, val: &String, r: &mut usize, c: &mut usize) -> bool{
        for row in 0..self.board.len(){
            for col in 0..self.board[row].len(){
                if self.board[row][col].eq(val){
                    *r = row;
                    *c = col;
                    return true;
                }
            }
        }
        false
    }
    fn mark_board(&mut self, row: &usize, col: &usize){
        if self.board[*row][*col].starts_with('x'){
            return
        }
        self.board[*row][*col] = String::from("x") + &self.board[*row][*col];
    }
}
impl fmt::Display for BingoBoard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        for row in &self.board{
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
//    for board in bingo_boards {
//        println!("{:#}", board);
//    }
    Ok(())
}
// --------------------------------------------------------------------------

//part1(&lines);
//part2(&lines);
