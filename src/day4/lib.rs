use std::fs;

struct Assignment {
    lower_section: u32,
    upper_section: u32,
}

impl Assignment {
    fn new(sections: &str) -> Self {
        let sections: Vec<&str> = sections.split('-').take(2).collect();

        Self {
            lower_section: sections[0].parse().unwrap(),
            upper_section: sections[1].parse().unwrap(),
        }
    }

    fn len(&self) -> usize {
        (self.lower_section..=self.upper_section).count()
    }

    fn fully_contains(&self, assignment: &Self) -> bool {
        if self.len() < assignment.len() {
            return false;
        }

        self.lower_section <= assignment.lower_section
            && self.upper_section >= assignment.upper_section
    }

    fn overlaps(&self, assignment: &Self) -> bool {
        assignment.lower_section <= self.upper_section
            && assignment.upper_section >= self.lower_section
    }
}
pub struct Day4 {}

impl Day4 {
    pub fn run() {
        let filename = "src/day4/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let mut fully_contains_counter = 0;

        for line in data.split('\n') {
            let assignments: Vec<&str> = line.split(',').take(2).collect();

            let assignment_a = Assignment::new(assignments[0]);
            let assignment_b = Assignment::new(assignments[1]);

            let fully_contains = if assignment_a.len() >= assignment_b.len() {
                assignment_a.fully_contains(&assignment_b)
            } else {
                assignment_b.fully_contains(&assignment_a)
            };

            if fully_contains {
                fully_contains_counter += 1;
            }
        }

        println!(
            "Part 1 - Total fully contains pairs: {}",
            fully_contains_counter
        );

        // Part 2
        let mut overlaps_counter = 0;

        for line in data.split('\n') {
            let assignments: Vec<&str> = line.split(',').take(2).collect();

            let assignment_a = Assignment::new(assignments[0]);
            let assignment_b = Assignment::new(assignments[1]);

            let overlaps = assignment_a.overlaps(&assignment_b);

            if overlaps {
                overlaps_counter += 1;
            }
        }

        println!("Part 2 - Total overlap pairs: {}", overlaps_counter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let assignment = Assignment::new("2-4");
        assert_eq!(assignment.len(), 3);

        let assignment = Assignment::new("4-5");
        assert_eq!(assignment.len(), 2);

        let assignment = Assignment::new("6-6");
        assert_eq!(assignment.len(), 1);
    }

    #[test]
    fn test_assignment_fully_contains() {
        let assignment_a = Assignment::new("2-4");
        let assignment_b = Assignment::new("6-8");

        assert!(!assignment_a.fully_contains(&assignment_b));

        let assignment_a = Assignment::new("2-6");
        let assignment_b = Assignment::new("4-8");

        assert!(!assignment_a.fully_contains(&assignment_b));

        let assignment_a = Assignment::new("2-8");
        let assignment_b = Assignment::new("3-7");

        assert!(assignment_a.fully_contains(&assignment_b));
        assert!(!assignment_b.fully_contains(&assignment_a));

        let assignment_a = Assignment::new("2-3");
        let assignment_b = Assignment::new("2-3");

        assert!(assignment_a.fully_contains(&assignment_b));
    }

    #[test]
    fn test_assignment_overlaps() {
        let assignment_a = Assignment::new("2-4");
        let assignment_b = Assignment::new("6-8");

        assert!(!assignment_a.overlaps(&assignment_b));

        let assignment_a = Assignment::new("2-3");
        let assignment_b = Assignment::new("4-5");

        assert!(!assignment_a.overlaps(&assignment_b));

        let assignment_a = Assignment::new("5-7");
        let assignment_b = Assignment::new("7-9");

        assert!(assignment_a.overlaps(&assignment_b));

        let assignment_a = Assignment::new("2-8");
        let assignment_b = Assignment::new("3-7");

        assert!(assignment_a.overlaps(&assignment_b));

        let assignment_a = Assignment::new("6-6");
        let assignment_b = Assignment::new("4-6");

        assert!(assignment_a.overlaps(&assignment_b));

        let assignment_a = Assignment::new("2-6");
        let assignment_b = Assignment::new("4-8");

        assert!(assignment_a.overlaps(&assignment_b));
    }
}
