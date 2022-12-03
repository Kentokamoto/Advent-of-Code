use std::collections::HashSet;

pub struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    pub fn new() -> Rucksack {
        Rucksack {
            compartment1: HashSet::new(),
            compartment2: HashSet::new(),
        }
    }
    pub fn add_to_compartment1(&mut self, item: char) {
        self.compartment1.insert(item);
    }
    pub fn add_to_compartment2(&mut self, item: char) {
        self.compartment2.insert(item);
    }
    pub fn find_duplicate(&self) -> Result<char, &str> {
        for item in &self.compartment1 {
            if self.compartment2.contains(&item) {
                return Ok(*item);
            }
        }
        Err("Could not find duplicates")
    }
}

#[cfg(test)]
mod rucksack_test {
    use super::*;

    #[test]
    fn test_duplicate() {
        let mut rucksack = Rucksack::new();
        let test_string = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let compartment_size = test_string.len() / 2;
        let first_half = &test_string[0..compartment_size];
        let second_half = &test_string[compartment_size..];
        for item_index in 0..compartment_size {
            let item = first_half.chars().nth(item_index).unwrap();
            rucksack.add_to_compartment1(item);
            let item = second_half.chars().nth(item_index).unwrap();
            rucksack.add_to_compartment2(item);
        }
        // '`' is the character before 'a' in ascii
        assert!(rucksack.find_duplicate().unwrap() as i32 - '`' as i32 == 16);
    }
}
