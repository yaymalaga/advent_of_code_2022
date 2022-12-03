use std::collections::HashSet;
use std::fs;

struct Compartment {
    items: HashSet<char>,
}

impl Compartment {
    fn new(items: &str) -> Self {
        let mut data = HashSet::new();

        items.chars().for_each(|x| {
            data.insert(x);
        });

        Self { items: data }
    }

    fn split(items: &str) -> [Self; 2] {
        let half = items.len() / 2;

        let compartment_a = Compartment::new(&items[..half]);
        let compartment_b = Compartment::new(&items[half..]);

        [compartment_a, compartment_b]
    }

    fn get_common_items(&self, compartment: &Compartment) -> Self {
        let mut common_items = HashSet::new();

        compartment.items.iter().for_each(|x| {
            if self.items.contains(x) {
                common_items.insert(*x);
            }
        });

        Self {
            items: common_items,
        }
    }

    fn calculate_priority(&self) -> u32 {
        self.items.iter().map(Self::_get_item_priority).sum()
    }

    fn _get_item_priority(item: &char) -> u32 {
        match item {
            'a'..='z' => *item as u32 - 96, // a is 97 in ASCII -> 1,
            'A'..='Z' => *item as u32 - 38, // A is 65 in ASCII -> 27,
            _ => panic!("Invalid item"),
        }
    }
}
pub struct Day3 {}

impl Day3 {
    pub fn run() {
        let filename = "src/day3/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let mut total_priority = 0;

        for line in data.split('\n') {
            let [compartment_a, compartment_b] = Compartment::split(line);

            let common_items = compartment_a.get_common_items(&compartment_b);
            
            total_priority += common_items.calculate_priority();
        }

        println!("Part 1 - Total priority is: {}", total_priority);

        // Part 2
        let mut total_priority = 0;
        let mut groups = Vec::with_capacity(3);

        for line in data.split('\n') {
            groups.push(Compartment::new(line));

            if groups.len() == 3 {
                let initial_common_items = groups[0].get_common_items(&groups[1]);
                let common_items = initial_common_items.get_common_items(&groups[2]);

                total_priority += common_items.calculate_priority();

                groups.clear();
            }
        }

        println!("Part 2 - Total groups priority is: {}", total_priority);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_items() {
        let [compartment_a, compartment_b] = Compartment::split("vJrwpWtwJgWrhcsFMMfFFhFp");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("p").items);

        let [compartment_a, compartment_b] = Compartment::split("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("L").items);

        let [compartment_a, compartment_b] = Compartment::split("PmmdzqPrVvPwwTWBwg");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("P").items);

        let [compartment_a, compartment_b] = Compartment::split("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("v").items);

        let [compartment_a, compartment_b] = Compartment::split("ttgJtRGJQctTZtZT");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("t").items);

        let [compartment_a, compartment_b] = Compartment::split("CrZsJsPPZsGzwwsLwLmpwMDw");
        let common_items = compartment_a.get_common_items(&compartment_b);
        assert_eq!(common_items.items, Compartment::new("s").items);
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(Compartment::_get_item_priority(&'p'), 16);
        assert_eq!(Compartment::_get_item_priority(&'L'), 38);
        assert_eq!(Compartment::_get_item_priority(&'P'), 42);
        assert_eq!(Compartment::_get_item_priority(&'v'), 22);
        assert_eq!(Compartment::_get_item_priority(&'t'), 20);
        assert_eq!(Compartment::_get_item_priority(&'s'), 19);
    }

    #[test]
    fn test_total_priority() {
        let compartment = Compartment::new("pLPvts");
        assert_eq!(compartment.calculate_priority(), 157);
    }
}
