use std::result::Result;

#[derive(PartialEq, Eq, Debug)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn for_opponent(input: &str) -> Result<Play, &str> {
        match input {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err("Not Valid"),
        }
    }
    fn for_self(input: &str) -> Result<Play, &str> {
        match input {
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            _ => Err("Not Valid"),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub enum Intent {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Intent {
    fn for_self(input: &str) -> Result<Intent, &str> {
        match input {
            "X" => Ok(Intent::Lose),
            "Y" => Ok(Intent::Draw),
            "Z" => Ok(Intent::Win),
            _ => Err("Not Valid"),
        }
    }
}
#[derive(Debug)]
pub struct Strategy {
    opponent_move: Play,
    self_move: Play,
}

impl Strategy {
    pub fn new(opponent: &str, me: &str) -> Strategy {
        Strategy {
            opponent_move: Play::for_opponent(opponent).unwrap(),
            self_move: Play::for_self(me).unwrap(),
        }
    }
    pub fn result(&self) -> i32 {
        let win = 6;
        let draw = 3;
        let lose = 0;
        let player_point = self.self_move as i32;
        if self.opponent_move == Play::Rock && self.self_move == Play::Rock {
            return player_point + draw;
        } else if self.opponent_move == Play::Rock && self.self_move == Play::Paper {
            return player_point + win;
        } else if self.opponent_move == Play::Rock && self.self_move == Play::Scissors {
            return player_point + lose;
        } else if self.opponent_move == Play::Paper && self.self_move == Play::Rock {
            return player_point + lose;
        } else if self.opponent_move == Play::Paper && self.self_move == Play::Paper {
            return player_point + draw;
        } else if self.opponent_move == Play::Paper && self.self_move == Play::Scissors {
            return player_point + win;
        } else if self.opponent_move == Play::Scissors && self.self_move == Play::Rock {
            return player_point + win;
        } else if self.opponent_move == Play::Scissors && self.self_move == Play::Paper {
            return player_point + lose;
        } else {
            // Scissors vs Scissors
            return player_point + draw;
        }
    }
}

#[derive(Debug)]
pub struct Strategy2 {
    opponent_move: Play,
    self_move: Intent,
}

impl Strategy2 {
    pub fn new(opponent: &str, me: &str) -> Strategy2 {
        Strategy2 {
            opponent_move: Play::for_opponent(opponent).unwrap(),
            self_move: Intent::for_self(me).unwrap(),
        }
    }
    pub fn result(&self) -> i32 {
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        let player_result = self.self_move as i32;
        if self.opponent_move == Play::Rock && self.self_move == Intent::Win {
            return player_result + paper;
        } else if self.opponent_move == Play::Rock && self.self_move == Intent::Lose {
            return player_result + scissors;
        } else if self.opponent_move == Play::Rock && self.self_move == Intent::Draw {
            return player_result + rock;
        } else if self.opponent_move == Play::Paper && self.self_move == Intent::Win {
            return player_result + scissors;
        } else if self.opponent_move == Play::Paper && self.self_move == Intent::Lose {
            return player_result + rock;
        } else if self.opponent_move == Play::Paper && self.self_move == Intent::Draw {
            return player_result + paper;
        } else if self.opponent_move == Play::Scissors && self.self_move == Intent::Win {
            return player_result + rock;
        } else if self.opponent_move == Play::Scissors && self.self_move == Intent::Lose {
            return player_result + paper;
        } else {
            // End in a draw
            return player_result + scissors;
        }
    }
}

#[cfg(test)]
mod strategy1_tests {
    use super::*;
    #[test]
    fn test_result() {
        let game1 = Strategy::new(&String::from("A"), &String::from("X"));
        assert_eq!(game1.result(), 4);
        let game2 = Strategy::new(&String::from("A"), &String::from("Y"));
        assert_eq!(game2.result(), 8);
        let game3 = Strategy::new(&String::from("A"), &String::from("Z"));
        assert_eq!(game3.result(), 3);
        let game4 = Strategy::new(&String::from("B"), &String::from("X"));
        assert_eq!(game4.result(), 1);
        let game5 = Strategy::new(&String::from("B"), &String::from("Y"));
        assert_eq!(game5.result(), 5);
        let game6 = Strategy::new(&String::from("B"), &String::from("Z"));
        assert_eq!(game6.result(), 9);
        let game7 = Strategy::new(&String::from("C"), &String::from("X"));
        assert_eq!(game7.result(), 7);
        let game8 = Strategy::new(&String::from("C"), &String::from("Y"));
        assert_eq!(game8.result(), 2);
        let game9 = Strategy::new(&String::from("C"), &String::from("Z"));
        assert_eq!(game9.result(), 6);
    }

    #[test]
    fn test_case_1() {
        let game1 = Strategy::new(&String::from("A"), &String::from("Y"));
        let game2 = Strategy::new(&String::from("B"), &String::from("X"));
        let game3 = Strategy::new(&String::from("C"), &String::from("Z"));
        let r = game1.result() + game2.result() + game3.result();
        assert_eq!(r, 15);
    }
}
mod strategy2_tests {
    use super::*;
    #[test]
    fn test_result() {
        let game1 = Strategy2::new(&String::from("A"), &String::from("X"));
        assert_eq!(game1.result(), 3);
        let game2 = Strategy2::new(&String::from("A"), &String::from("Y"));
        assert_eq!(game2.result(), 4);
        let game3 = Strategy2::new(&String::from("A"), &String::from("Z"));
        assert_eq!(game3.result(), 8);
        let game4 = Strategy2::new(&String::from("B"), &String::from("X"));
        assert_eq!(game4.result(), 1);
        let game5 = Strategy2::new(&String::from("B"), &String::from("Y"));
        assert_eq!(game5.result(), 5);
        let game6 = Strategy2::new(&String::from("B"), &String::from("Z"));
        assert_eq!(game6.result(), 9);
        let game7 = Strategy2::new(&String::from("C"), &String::from("X"));
        assert_eq!(game7.result(), 2);
        let game8 = Strategy2::new(&String::from("C"), &String::from("Y"));
        assert_eq!(game8.result(), 6);
        let game9 = Strategy2::new(&String::from("C"), &String::from("Z"));
        assert_eq!(game9.result(), 7);
    }

    #[test]
    fn test_case_1() {
        let game1 = Strategy2::new(&String::from("A"), &String::from("Y"));
        let game2 = Strategy2::new(&String::from("B"), &String::from("X"));
        let game3 = Strategy2::new(&String::from("C"), &String::from("Z"));
        let r = game1.result() + game2.result() + game3.result();
        assert_eq!(r, 12);
    }
}
