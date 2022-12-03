use std::fs;

struct ElfList {
    list: Vec<Elf>,
}

impl ElfList {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn add_elf(&mut self, elf: Elf) {
        self.list.push(elf);
    }

    fn get_top_elf_calories(&self) -> u32 {
        self.list
            .iter()
            .map(|item| item.calories)
            .max()
            .unwrap()
    }

    fn get_top_3_calories(&self) -> u32 {
        let mut calories_per_elf: Vec<u32> = self
            .list
            .iter()
            .map(|item| item.calories)
            .collect();

        calories_per_elf.sort();

        calories_per_elf.iter().rev().take(3).sum()
    }
}
struct Elf {
    calories: u32,
}

impl Elf {
    fn new() -> Self {
        Self {
            calories: 0,
        }
    }

    fn add_calories(&mut self, calories: u32) {
        self.calories += calories;
    }
}

pub struct Day1 {}

impl Day1 {
    pub fn run() {
        let filename = "src/day1/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        let mut elf_list = ElfList::new();
        let mut elf = Elf::new();

        for line in data.split('\n') {
            if line.is_empty() {
                elf_list.add_elf(elf);
                elf = Elf::new();
            } else {
                elf.add_calories(line.parse::<u32>().unwrap());
            }
        }

        let max_calories: u32 = elf_list.get_top_elf_calories();

        println!("Part 1 - Max calories carried by an elf: {}", max_calories);

        let max_calories_top_3 = elf_list.get_top_3_calories();

        println!("Part 2 - Total calories from top 3: {}", max_calories_top_3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_elf_list() -> ElfList {
        let mut elf_list = ElfList::new();

        let mut elf = Elf::new();
        elf.add_calories(1000);
        elf.add_calories(2000);
        elf.add_calories(3000);
        
        elf_list.add_elf(elf);
        
        let mut elf = Elf::new();
        elf.add_calories(4000);

        elf_list.add_elf(elf);
        
        let mut elf = Elf::new();
        elf.add_calories(5000);
        elf.add_calories(6000);
        
        elf_list.add_elf(elf);

        let mut elf = Elf::new();
        elf.add_calories(7000);
        elf.add_calories(8000);
        elf.add_calories(9000);

        elf_list.add_elf(elf);
        
        let mut elf = Elf::new();
        elf.add_calories(10000);

        elf_list.add_elf(elf);

        elf_list
    }

    #[test]
    fn test_elf_calories() {
        let elf_list = generate_elf_list();
        
        assert_eq!(elf_list.list[0].calories, 6000);
        assert_eq!(elf_list.list[1].calories, 4000);
        assert_eq!(elf_list.list[2].calories, 11000);
        assert_eq!(elf_list.list[3].calories, 24000);
        assert_eq!(elf_list.list[4].calories, 10000);

    }

    #[test]
    fn test_top_elf_calories() {
        let elf_list = generate_elf_list();
        
        assert_eq!(elf_list.get_top_elf_calories(), 24000);
    }

    #[test]
    fn test_top_3_elf_calories() {
        let elf_list = generate_elf_list();
        
        assert_eq!(elf_list.get_top_3_calories(), 45000);
    }

}
