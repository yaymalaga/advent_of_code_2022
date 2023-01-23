use std::{fs, collections::HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    fn is_touching(&self, other: &Self) -> bool {
        ((self.x - 1)..=(self.x + 1)).contains(&other.x) && ((self.y - 1)..=(self.y + 1)).contains(&other.y)
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => {self.x -= 1},
            Direction::Up => {self.y += 1},
            Direction::Right => {self.x += 1},
            Direction::Down => {self.y -= 1},
        };
    }

    fn follow(&mut self, other: &Self) -> bool {
        if self.is_touching(other) {
            return false;
        }

        let delta_x = (other.x - self.x).clamp(-1, 1);
        let delta_y = (other.y - self.y).clamp(-1, 1);

        self.x += delta_x;
        self.y += delta_y;

        return true;
    }
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn parse(str: &str) -> Self {
        match str {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => panic!("No valid direction string")
        }
    }
}


pub struct Day9 {}

impl Day9 {
    pub fn run() {
        let filename = "src/day9/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        let mut head = Knot::new(0,0);
        let mut tail = Knot::new(0,0);

        let mut tail_positions = HashSet::new();
        tail_positions.insert(tail);

        for line in data.lines() {
            let mut line_split = line.split_whitespace();
            let step_direction = line_split.next().unwrap();
            let n_steps = line_split.next().unwrap();

            let step_direction = Direction::parse(step_direction);
            let n_steps = n_steps.parse().unwrap();

            for _ in 0..n_steps {
                head.step(&step_direction);
                
                let did_move = tail.follow(&mut head);
                
                if did_move {
                    tail_positions.insert(tail);
                }
            }
        }

        println!(
            "Part 1 - Visited positions: {}",
            tail_positions.len()
        );

        const KNOTS_LEN: usize = 10;
        let mut knots = [Knot::new(0, 0);KNOTS_LEN];
        
        let mut tail_positions = HashSet::new();
        tail_positions.insert(knots[0]);
        
        for line in data.lines() {
            let mut line_split = line.split_whitespace();
            let step_direction = line_split.next().unwrap();
            let n_steps = line_split.next().unwrap();

            let step_direction = Direction::parse(step_direction);
            let n_steps = n_steps.parse().unwrap();

            for _ in 0..n_steps {
                
                let head = knots.get_mut(0).unwrap();
                head.step(&step_direction);

                for index in 0..KNOTS_LEN-1 {
                    let head = knots[index];
                    let tail = knots.get_mut(index+1).unwrap();
                    
                    let did_move = tail.follow(&head);
                    if did_move && index == KNOTS_LEN-2 {
                        tail_positions.insert(*tail);
                    }
                }
            }
        }

        println!(
            "Part 2 - Visited positions: {}",
            tail_positions.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_touching() {
        let knot_a = Knot::new(-2, -1);
        let knot_b = Knot::new(-1, 0);
        assert!(knot_a.is_touching(&knot_b));

        let knot_a = Knot::new(0, 0);
        let knot_b = Knot::new(-1, 0);
        assert!(knot_a.is_touching(&knot_b));

        let knot_a = Knot::new(0, 0);
        let knot_b = Knot::new(-2, -1);
        assert!(!knot_a.is_touching(&knot_b));
    }

    #[test]
    fn test_step() {
        let mut knot = Knot::new(-1, 4);
        knot.step(&Direction::Up);
        knot.step(&Direction::Up);
        knot.step(&Direction::Right);
        knot.step(&Direction::Down);
        knot.step(&Direction::Left);
        knot.step(&Direction::Left);
        assert_eq!(knot.x, -2);
        assert_eq!(knot.y, 5);
    }

    #[test]
    fn test_follow() {
        let mut knot_a = Knot::new(0, 0);
        let mut knot_b = Knot::new(0, 0);

        knot_b.step(&Direction::Up);
        assert!(!knot_a.follow(&mut knot_b));
        assert_eq!(knot_a.x, 0);
        assert_eq!(knot_a.y, 0);

        knot_b.step(&Direction::Right);
        assert!(!knot_a.follow(&mut knot_b));
        assert_eq!(knot_a.x, 0);
        assert_eq!(knot_a.y, 0);

        knot_b.step(&Direction::Up);
        assert!(knot_a.follow(&mut knot_b));
        assert_eq!(knot_a.x, 1);
        assert_eq!(knot_a.y, 1);
    }
}