use std::{cmp::min, fs};

struct Forest {
    map: Vec<Vec<Tree>>,
}

impl Forest {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn add_line(&mut self, line: Vec<Tree>) {
        self.map.push(line);
    }

    fn check_visibility(&self, row: usize, col: usize) -> bool {
        let max_column = self.max_col();
        let max_row = self.max_row();

        // Perimeter is always visible
        if row == 0 || row == max_row || col == 0 || col == max_column {
            return true;
        }

        Self::check_visibility_left(self, row, col)
            || Self::check_visibility_right(self, row, col)
            || Self::check_visibility_up(self, row, col)
            || Self::check_visibility_down(self, row, col)
    }

    fn count_trees_view(&self, row: usize, col: usize) -> u32 {
        Self::count_trees_view_left(self, row, col)
            * Self::count_trees_view_right(self, row, col)
            * Self::count_trees_view_up(self, row, col)
            * Self::count_trees_view_down(self, row, col)
    }

    fn check_visibility_left(&self, row: usize, col: usize) -> bool {
        let height = self.get_tree_height(row, col);
        (0..col)
            .map(|i| self.get_tree_height(row,i))
            .all(|tree_height| tree_height < height)
    }

    fn count_trees_view_left(&self, row: usize, col: usize) -> u32 {
        let height = self.get_tree_height(row, col);
        
        let mut counter = 0;
        for i in (0..col).rev() {
            let tree_height = self.get_tree_height(row,i);
            
            counter += 1;

            if tree_height >= height {
                break;
            }
        }

        counter
    }

    fn check_visibility_right(&self, row: usize, col: usize) -> bool {
        let height = self.get_tree_height(row, col);
        let max_column = self.max_col();

        (min(col + 1, max_column)..=max_column)
            .map(|i| self.get_tree_height(row,i))
            .all(|tree_height| tree_height < height)
    }

    fn count_trees_view_right(&self, row: usize, col: usize) -> u32 {
        let height = self.get_tree_height(row, col);
        let max_column = self.max_col();
        
        let mut counter = 0;
        for i in min(col + 1, max_column)..=max_column {
            let tree_height = self.get_tree_height(row,i);
            
            counter += 1;
            
            if tree_height >= height {
                break;
            }
        }

        counter
    }

    fn check_visibility_up(&self, row: usize, col: usize) -> bool {
        let height = self.get_tree_height(row, col);
        (0..row)
            .map(|i| self.get_tree_height(i, col))
            .all(|tree_height| tree_height < height)
    }

    fn count_trees_view_up(&self, row: usize, col: usize) -> u32 {
        let height = self.get_tree_height(row, col);
        
        let mut counter = 0;
        for i in (0..row).rev() {
            let tree_height = self.get_tree_height(i,col);
            
            if tree_height <= height {
                counter += 1;

                if tree_height >= height {
                    break;
                }
            } 
        }

        counter
    }

    fn check_visibility_down(&self, row: usize, col: usize) -> bool {
        let height = self.get_tree_height(row, col);
        let max_row = self.max_row();

        (min(row + 1, max_row)..=max_row)
            .map(|i| self.get_tree_height(i, col))
            .all(|tree_height| tree_height < height)
    }

    fn count_trees_view_down(&self, row: usize, col: usize) -> u32 {
        let height = self.get_tree_height(row, col);
        let max_row = self.max_row();
        
        let mut counter = 0;
        for i in min(row + 1, max_row)..=max_row {
            let tree_height = self.get_tree_height(i,col);
            
            if tree_height <= height {
                counter += 1;

                if tree_height >= height {
                    break;
                }
            } 
        }

        counter
    }
    
    fn count_visible(&self) -> usize {
        let rows = self.map.len();
        let cols = self.map[0].len();

        let mut count = 0;
        for row in 0..rows {
            for col in 0..cols {
                if self.check_visibility(row, col) {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_max_trees_view(&self) -> u32 {
        let rows = self.map.len();
        let cols = self.map[0].len();

        let mut trees_view_result = 0;
        for row in 0..rows {
            for col in 0..cols {
               let trees_view = self.count_trees_view(row, col);

               if trees_view > trees_view_result {
                trees_view_result = trees_view;
               }
            }
        }

        trees_view_result
    }

    #[inline]
    fn max_col(&self) -> usize {
        self.map[0].len() - 1
    }

    #[inline]
    fn max_row(&self) -> usize {
        self.map.len() - 1
    }

    #[inline]
    fn get_tree_height(&self, row: usize, col: usize) -> u32 {
        self.map[row][col].height
    }

    
}

struct Tree {
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self { height }
    }
}

pub struct Day8 {}

impl Day8 {
    pub fn run() {
        let filename = "src/day8/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let mut forest = Forest::new();

        for line in data.lines() {
            let forest_line = line
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .map(Tree::new)
                .collect::<Vec<Tree>>();

            forest.add_line(forest_line);
        }

        println!(
            "Part 1 - Number of visible trees: {}",
            forest.count_visible()
        );

        println!(
            "Part 2 - Max scenic score is: {}",
            forest.count_max_trees_view()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_forest() -> Forest {
        Forest {
            map: vec![
                vec![Tree::new(3), Tree::new(0), Tree::new(3), Tree::new(7), Tree::new(3)],
                vec![Tree::new(2), Tree::new(5), Tree::new(5), Tree::new(1), Tree::new(2)],
                vec![Tree::new(6), Tree::new(5), Tree::new(3), Tree::new(3), Tree::new(2)],
                vec![Tree::new(3), Tree::new(3), Tree::new(5), Tree::new(4), Tree::new(9)],
                vec![Tree::new(3), Tree::new(5), Tree::new(3), Tree::new(9), Tree::new(0)],
            ],
        }
    }

    #[test]
    fn test_visibility() {
        let forest = generate_forest();
        
        // Check inner
        assert!(forest.check_visibility_left(1, 1));
        assert!(forest.check_visibility_up(1, 1));
        assert!(!forest.check_visibility_right(1, 1));
        assert!(!forest.check_visibility_down(1, 1));
        assert!(forest.check_visibility(1, 1));

        assert!(!forest.check_visibility_left(2, 2));
        assert!(!forest.check_visibility_up(2, 2));
        assert!(!forest.check_visibility_right(2, 2));
        assert!(!forest.check_visibility_down(2, 2));
        assert!(!forest.check_visibility(2, 2));

        assert!(!forest.check_visibility(3, 1));
        assert!(forest.check_visibility(3, 2));
        assert!(!forest.check_visibility(3, 3));

        // Check edges
        assert!(forest.check_visibility(0, 0));
        assert!(forest.check_visibility(0, 4));
        assert!(forest.check_visibility(4, 0));
        assert!(forest.check_visibility(4, 4));

        // Check perimeter
        assert!(forest.check_visibility(0, 2));
        assert!(forest.check_visibility(3, 0));
        assert!(forest.check_visibility(2, 4));
        assert!(forest.check_visibility(4, 3));
    }

    #[test]
    fn test_count_view() {
        let forest = generate_forest();
        
        assert_eq!(forest.count_trees_view_left(1, 2), 1);
        assert_eq!(forest.count_trees_view_up(1, 2), 1);
        assert_eq!(forest.count_trees_view_right(1, 2), 2);
        assert_eq!(forest.count_trees_view_down(1, 2), 2);
        assert_eq!(forest.count_trees_view(1, 2), 4);

        assert_eq!(forest.count_trees_view_left(3, 2), 2);
        assert_eq!(forest.count_trees_view_up(3, 2), 2);
        assert_eq!(forest.count_trees_view_right(3, 2), 2);
        assert_eq!(forest.count_trees_view_down(3, 2), 1);
        assert_eq!(forest.count_trees_view(3, 2), 8);

       
    }
}
