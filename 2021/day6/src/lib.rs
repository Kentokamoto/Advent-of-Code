use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct LanternFish {
    timer: u32,
    default: u32
}

impl LanternFish{
    fn new(timer: u32, default: u32) -> LanternFish{
        LanternFish{
            timer: timer,
            default: default
        }
    }

    fn decrement(&mut self){
        self.timer -= 1;
    }

    fn reset(&mut self){
        self.timer = self.default;
    }

    fn time_to_spawn(&self) -> bool{
        self.timer <=0 
    }
}


pub fn new(file_name: &String) -> Result<Vec<LanternFish>, &str> {
    let file = match File::open(&file_name.trim()) {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut buf_reader = BufReader::new(file);

    let mut output = Vec::new();
    let mut line = String::new();
    let len = buf_reader.read_line(&mut line).unwrap();
    let timers: Vec<&str> = line.split(",").collect();
    for timer in timers {
        output.push(LanternFish::new(timer.trim().parse::<u32>().unwrap(), 6));
    }
    Ok(output)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn decrement() {
    let mut test1 = LanternFish::new(6, 6);
    test1.decrement();
    assert_eq!(5, test1.timer);
    }
    
    #[test]
    fn reset() {
    let mut test1 = LanternFish::new(3, 6);
    test1.reset();
    assert_eq!(6, test1.timer);
    }

    #[test]
    fn time_to_spawn(){
        let test1 = LanternFish::new(0,6);

        assert_eq!(true, test1.time_to_spawn());

        let test2 = LanternFish::new(4,6);
        assert_eq!(false, test2.time_to_spawn());
    }
}
