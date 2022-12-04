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
#[derive(Debug)]
pub struct ElfGroup {
    rucksack1: HashSet<char>,
    rucksack2: HashSet<char>,
    rucksack3: HashSet<char>,
}
impl ElfGroup {
    pub fn new() -> ElfGroup {
        ElfGroup {
            rucksack1: HashSet::new(),
            rucksack2: HashSet::new(),
            rucksack3: HashSet::new(),
        }
    }
    pub fn add_to_rucksack(&mut self, sack_num: &i32, item: char) -> Result<bool, &str> {
        match sack_num {
            1 => Ok(self.rucksack1.insert(item)),
            2 => Ok(self.rucksack2.insert(item)),
            3 => Ok(self.rucksack3.insert(item)),
            _ => Err("It needs to be a value between 1 and 3"),
        }
    }
    pub fn find_badge(&self) -> Result<char, &str> {
        let mut rucksack_1_and_2: Vec<char> = Vec::new();
        for item in &self.rucksack1 {
            if self.rucksack2.contains(&item) {
                rucksack_1_and_2.push(*item);
            }
        }
        for item in rucksack_1_and_2 {
            if self.rucksack3.contains(&item) {
                return Ok(item);
            }
        }
        Err("Couldn't find a badge")
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
#[cfg(test)]
mod elf_group_test {
    use super::*;
    #[test]
    fn test_find_badge() {
        let sack1 = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let sack2 = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let sack3 = String::from("PmmdzqPrVvPwwTWBwg");
        let mut eg = ElfGroup::new();
        for c in sack1.chars() {
            eg.add_to_rucksack(1, c).unwrap();
        }
        for c in sack2.chars() {
            eg.add_to_rucksack(2, c).unwrap();
        }
        for c in sack3.chars() {
            eg.add_to_rucksack(3, c).unwrap();
        }
        assert!(eg.find_badge().unwrap() as i32 - '`' as i32 == 18)
    }
}
