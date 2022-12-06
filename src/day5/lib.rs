use std::fs;

struct CratesPort {
    stacks: Vec<Vec<Crate>>,
}

impl CratesPort {
    fn new(number_stacks: usize) -> Self {
        let mut port = Self {
            stacks: Vec::with_capacity(number_stacks),
        };

        for _ in 0..number_stacks {
            port.stacks.push(Vec::new());
        }

        port
    }

    fn add_crate(&mut self, item: Crate, stack_index: usize) {
        self.stacks[stack_index].push(item);
    }

    fn individual_movement(&mut self, action: Action) {
        for _ in 1..=action.amount {
            let item = self.stacks[action.origin].pop().unwrap();
            self.stacks[action.destiny].push(item)
        }
    }

    fn group_movement(&mut self, action: Action) {
        let mut crates = Vec::new();

        for _ in 1..=action.amount {
            let item = self.stacks[action.origin].pop().unwrap();
            crates.push(item);
        }

        for item in crates.into_iter().rev() {
            self.stacks[action.destiny].push(item)
        }
    }

    fn get_upper_crates_msg(&self) -> String {
        let mut msg = String::new();

        for stack in &self.stacks {
            match stack.last() {
                Some(Crate { id }) => msg.push_str(id),
                None => msg.push(' '),
            };
        }

        msg
    }
}

struct Crate {
    id: String,
}

impl Crate {
    fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, PartialEq)]
struct Action {
    origin: usize,
    destiny: usize,
    amount: usize,
}

impl Action {
    fn new(origin: usize, destiny: usize, amount: usize) -> Self {
        // Puzzle inputs start from 1
        Self {
            origin: origin - 1,
            destiny: destiny - 1,
            amount,
        }
    }
}

pub struct Day5 {}

impl Day5 {
    pub fn run() {
        let filename = "src/day5/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let number_stacks = Self::get_number_stacks(&data);
        let mut crates_port = CratesPort::new(number_stacks);

        let mut crates = Vec::new();
        let mut actions = Vec::new();

        for line in data.split('\n') {
            if line.contains('[') {
                let crates_data = Self::parse_crates_line(line);
                crates.push(crates_data);
            } else if line.contains("move") {
                let action = Self::parse_action_line(line);
                actions.push(action);
            } else {
                continue;
            }
        }

        for (index, item) in crates.into_iter().rev().flatten() {
            crates_port.add_crate(item, index)
        }

        for action in actions {
            crates_port.individual_movement(action);
        }

        println!(
            "Part 1 - Msg from crates on top: {}",
            crates_port.get_upper_crates_msg()
        );

        // Part 2
        let number_stacks = Self::get_number_stacks(&data);
        let mut crates_port = CratesPort::new(number_stacks);

        let mut crates = Vec::new();
        let mut actions = Vec::new();

        for line in data.split('\n') {
            if line.contains('[') {
                let crates_data = Self::parse_crates_line(line);
                crates.push(crates_data);
            } else if line.contains("move") {
                let action = Self::parse_action_line(line);
                actions.push(action);
            } else {
                continue;
            }
        }

        for (index, item) in crates.into_iter().rev().flatten() {
            crates_port.add_crate(item, index)
        }

        for action in actions {
            crates_port.group_movement(action);
        }

        println!(
            "Part 2 - Msg from crates on top: {}",
            crates_port.get_upper_crates_msg()
        );
    }

    fn parse_crates_line(line: &str) -> Vec<(usize, Crate)> {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, id)| !id.is_whitespace())
            .map(|(index, id)| (index, Crate::new(id.to_string())))
            .collect()
    }

    fn parse_action_line(line: &str) -> Action {
        let action: Vec<usize> = line
                    .replace("move ", "")
                    .replace(" from ", " ")
                    .replace(" to ", " ")
                    .split_whitespace()
                    .take(3)
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

        Action::new(action[1], action[2], action[0])
    }

    fn get_number_stacks(data: &str) -> usize {
        let first_line_len = data.split('\n').next().unwrap().len();
        (first_line_len + 1) / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_stacks() {
        let data = "[D]        ";
        let stacks = Day5::get_number_stacks(data);

        assert!(data.contains('['));
        assert_eq!(stacks, 3);

        let data = "[M] [H]         [N]                ";
        let stacks = Day5::get_number_stacks(data);

        assert!(data.contains('['));
        assert_eq!(stacks, 9);
    }

    #[test]
    fn test_parse_action() {
        let line = "move 1 from 2 to 1";
        let action = Day5::parse_action_line(line);

        assert_eq!(action, Action::new(2, 1, 1));
        
        let line = "move 11 from 4 to 1";
        let action = Day5::parse_action_line(line);
        assert_eq!(action, Action::new(4, 1, 11));
    }

    #[test]
    fn test_parse_crates() {
        let line = "[N] [C]    ";
        let crates = Day5::parse_crates_line(line);

        assert_eq!(crates.len(), 2);
        assert_eq!(crates[0].0, 0);
        assert_eq!(crates[0].1.id, "N");
        assert_eq!(crates[1].0, 1);
        assert_eq!(crates[1].1.id, "C");

        let line = "        [Z]";
        let crates = Day5::parse_crates_line(line);

        assert_eq!(crates.len(), 1);
        assert_eq!(crates[0].0, 2);
        assert_eq!(crates[0].1.id, "Z");

        let line = "    [C] [D]";
        let crates = Day5::parse_crates_line(line);

        assert_eq!(crates.len(), 2);
        assert_eq!(crates[0].0, 1);
        assert_eq!(crates[0].1.id, "C");
        assert_eq!(crates[1].0, 2);
        assert_eq!(crates[1].1.id, "D");
    }

    #[test]
    fn test_crates_port_add() {
        let mut port = CratesPort::new(3);
        
        let crates = Day5::parse_crates_line("[N] [C]    ");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        assert_eq!(port.stacks[0].len(), 1);
        assert_eq!(port.stacks[1].len(), 1);
        assert_eq!(port.stacks[2].len(), 0);
        assert_eq!(port.stacks[0].last().unwrap().id, "N");
        assert_eq!(port.stacks[1].last().unwrap().id, "C");

        let crates = Day5::parse_crates_line("    [A] [Z]");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        assert_eq!(port.stacks[0].len(), 1);
        assert_eq!(port.stacks[1].len(), 2);
        assert_eq!(port.stacks[2].len(), 1);
        assert_eq!(port.stacks[0].last().unwrap().id, "N");
        assert_eq!(port.stacks[1].last().unwrap().id, "A");
        assert_eq!(port.stacks[1].first().unwrap().id, "C");
        assert_eq!(port.stacks[2].last().unwrap().id, "Z");

    }

    #[test]
    fn test_crates_individual_movement() {
        let mut port = CratesPort::new(3);
        
        let crates = Day5::parse_crates_line("[N] [C]    ");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        let crates = Day5::parse_crates_line("    [A] [Z]");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        let action = Action::new(2, 3, 2);
        port.individual_movement(action);

        assert_eq!(port.stacks[0].len(), 1);
        assert_eq!(port.stacks[1].len(), 0);
        assert_eq!(port.stacks[2].len(), 3);
        assert_eq!(port.stacks[0].last().unwrap().id, "N");
        assert_eq!(port.stacks[2].last().unwrap().id, "C");
        assert_eq!(port.stacks[2].first().unwrap().id, "Z");

        let action = Action::new(1, 3, 1);
        port.individual_movement(action);

        assert_eq!(port.stacks[0].len(), 0);
        assert_eq!(port.stacks[1].len(), 0);
        assert_eq!(port.stacks[2].len(), 4);
        assert_eq!(port.stacks[2].last().unwrap().id, "N");
        assert_eq!(port.stacks[2].first().unwrap().id, "Z");
    }

    #[test]
    fn test_upper_msg() {
        let mut port = CratesPort::new(3);

        let crates_data = vec![
            Day5::parse_crates_line("    [D]    "),
            Day5::parse_crates_line("[N] [C]    "),
            Day5::parse_crates_line("[Z] [M] [P]"),
        ];
        for (index, item) in crates_data.into_iter().flatten() {
            port.add_crate(item, index);
        }

        assert_eq!(port.get_upper_crates_msg(), "ZMP");
    }

    #[test]
    fn test_crates_group_movement() {
        let mut port = CratesPort::new(3);
        
        let crates = Day5::parse_crates_line("[N] [C]    ");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        let crates = Day5::parse_crates_line("    [A] [Z]");
        for (index, item) in crates {
            port.add_crate(item, index);
        }

        let action = Action::new(2, 3, 2);
        port.group_movement(action);

        assert_eq!(port.stacks[0].len(), 1);
        assert_eq!(port.stacks[1].len(), 0);
        assert_eq!(port.stacks[2].len(), 3);
        assert_eq!(port.stacks[0].last().unwrap().id, "N");
        assert_eq!(port.stacks[2].last().unwrap().id, "A");
        assert_eq!(port.stacks[2].first().unwrap().id, "Z");

        let action = Action::new(1, 3, 1);
        port.group_movement(action);

        assert_eq!(port.stacks[0].len(), 0);
        assert_eq!(port.stacks[1].len(), 0);
        assert_eq!(port.stacks[2].len(), 4);
        assert_eq!(port.stacks[2].last().unwrap().id, "N");
        assert_eq!(port.stacks[2].first().unwrap().id, "Z");
    }
}
