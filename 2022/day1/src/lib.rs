pub struct Elf {
    items: Vec<i32>,
    total: i32,
}
impl Elf {
    pub fn new() -> Elf {
        Elf {
            items: Vec::new(),
            total: 0,
        }
    }

    pub fn add(&mut self, input: i32) {
        self.items.push(input);
        self.total += input;
    }

    pub fn get_total(&self) -> i32 {
        self.total
    }
}
#[cfg(test)]
mod elf_tests {
    use super::*;
    #[test]
    fn test_add() {
        let mut elf1 = Elf::new();
        elf1.add(1000);
        elf1.add(2000);
        elf1.add(3000);
        assert_eq!(elf1.get_total(), 6000)
    }
}
