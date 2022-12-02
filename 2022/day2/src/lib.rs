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

#[cfg(test)]
mod game_tests {
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
