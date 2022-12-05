#[derive(Debug)]

pub struct CargoCrane {
    stacks: Vec<Vec<char>>,
}

impl CargoCrane {
    pub fn new() -> CargoCrane {
        CargoCrane { stacks: Vec::new() }
    }
    pub fn parse_input(&mut self, contents: &String) {
        let stack_size = contents.lines().next().unwrap().len() / 4 + 1;
        for _ in 0..stack_size {
            self.stacks.push(Vec::new());
        }
        for line in contents.lines() {
            // Format of string is set up in a way where every 1 + 4n is an item
            for i in 0..stack_size {
                let index = 1 + 4 * i;
                let crate_name = line.chars().nth(index).unwrap();
                if crate_name != ' ' && !crate_name.is_numeric() {
                    self.stacks[i].insert(0, crate_name);
                }
            }
        }
    }
    pub fn move_crate(&mut self, move_num: &i32, from_stack: &usize, to_stack: &usize) {
        for _ in 0..*move_num {
            if self.stacks[*from_stack].is_empty() {
                return;
            }
            let crate_name = self.stacks[*from_stack].pop().unwrap();
            self.stacks[*to_stack].push(crate_name);
        }
    }
    pub fn move_crate_2(&mut self, move_num: &i32, from_stack: &usize, to_stack: &usize) {
        let mut temp_stack = Vec::new();
        for _ in 0..*move_num {
            if self.stacks[*from_stack].is_empty() {
                return;
            }
            temp_stack.insert(0, self.stacks[*from_stack].pop().unwrap());
        }
        self.stacks[*to_stack].append(&mut temp_stack);
    }
    pub fn get_top(&self) -> String {
        let mut output = String::new();
        for stack in &self.stacks {
            output += stack.last().unwrap().to_string().as_str();
        }
        output
    }
}

#[cfg(test)]
mod cargo_crane_tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let input = "    [D]    \n\
                     [N] [C]    \n\
                     [Z] [M] [P]\n \
                      1   2   3 ";
        let mut cargo_crane = CargoCrane::new();
        cargo_crane.parse_input(&String::from(input));
        assert_eq!(cargo_crane.stacks.len(), 3);
        assert_eq!(cargo_crane.stacks[0].len(), 2);
        assert_eq!(cargo_crane.stacks[1].len(), 3);
        assert_eq!(cargo_crane.stacks[2].len(), 1);
    }
    #[test]
    fn test_move() {
        let input = "    [D]    \n\
                     [N] [C]    \n\
                     [Z] [M] [P]\n \
                      1   2   3 ";
        let moves = "move 1 from 2 to 1\n\
                     move 3 from 1 to 3\n\
                     move 2 from 2 to 1\n\
                     move 1 from 1 to 2";
        let mut cargo_crane = CargoCrane::new();
        cargo_crane.parse_input(&String::from(input));
        for line in moves.lines() {
            let mut sp = line.split_whitespace();
            let move_count = sp.nth(1).unwrap().parse::<i32>().unwrap();
            let from_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let to_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            cargo_crane.move_crate(&move_count, &from_crate, &to_crate);
        }
        assert_eq!(cargo_crane.get_top(), "CMZ");
        assert_eq!(cargo_crane.stacks[0].pop().unwrap(), 'C');
        assert_eq!(cargo_crane.stacks[1].pop().unwrap(), 'M');
        assert_eq!(cargo_crane.stacks[2].pop().unwrap(), 'Z');
    }
    #[test]
    fn test_move2() {
        let input = "    [D]    \n\
                     [N] [C]    \n\
                     [Z] [M] [P]\n \
                      1   2   3 ";
        let moves = "move 1 from 2 to 1\n\
                     move 3 from 1 to 3\n\
                     move 2 from 2 to 1\n\
                     move 1 from 1 to 2";
        let mut cargo_crane = CargoCrane::new();
        cargo_crane.parse_input(&String::from(input));
        for line in moves.lines() {
            let mut sp = line.split_whitespace();
            let move_count = sp.nth(1).unwrap().parse::<i32>().unwrap();
            let from_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let to_crate = sp.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            cargo_crane.move_crate_2(&move_count, &from_crate, &to_crate);
        }
        assert_eq!(cargo_crane.get_top(), "MCD");
        assert_eq!(cargo_crane.stacks[0].pop().unwrap(), 'M');
        assert_eq!(cargo_crane.stacks[1].pop().unwrap(), 'C');
        assert_eq!(cargo_crane.stacks[2].pop().unwrap(), 'D');
    }
}
