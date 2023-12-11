use std::fs;

struct Tree {
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Tree {
        Tree { height }
    }
}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn from_str(input: String) -> Forest {
        let mut trees: Vec<Vec<Tree>> = Vec::new();

        for line in input.lines() {
            let mut row: Vec<Tree> = Vec::new();
            for c in line.chars() {
                let height = c.to_digit(10).unwrap() as u32;
                row.push(Tree::new(height));
            }
            trees.push(row);
        }

        Forest { trees }
    }

    fn visible_trees_up(&self, row: usize, col: usize) -> u32 {
        let mut count: u32 = 0;
        let tree_height = self.trees[row][col].height;

        for row_idx in (0..row).rev() {
            let tree = &self.trees[row_idx][col];
            if tree.height >= tree_height {
                return count + 1;
            }

            count += 1;
        }

        count
    }

    fn visible_trees_down(&self, row: usize, col: usize) -> u32 {
        let mut count: u32 = 0;
        let tree_height = self.trees[row][col].height;

        for row_idx in row + 1..self.trees.len() {
            let tree = &self.trees[row_idx][col];
            if tree.height >= tree_height {
                return count + 1;
            }

            count += 1;
        }

        count
    }

    fn visible_trees_left(&self, row: usize, col: usize) -> u32 {
        let mut count: u32 = 0;
        let tree_height = self.trees[row][col].height;

        for col_idx in (0..col).rev() {
            let tree = &self.trees[row][col_idx];
            if tree.height >= tree_height {
                return count + 1;
            }

            count += 1;
        }

        count
    }

    fn visible_trees_right(&self, row: usize, col: usize) -> u32 {
        let mut count: u32 = 0;
        let tree_height = self.trees[row][col].height;

        for col_idx in col + 1..self.trees[row].len() {
            let tree = &self.trees[row][col_idx];

            if tree.height >= tree_height {
                return count + 1;
            }

            count += 1;
        }

        count
    }

    fn find_highest_scenic_score(&self) -> u32 {
        let mut highest_score = 0;

        for row in 1..self.trees.len() - 1 {
            for col in 1..self.trees[row].len() - 1 {
                let up = self.visible_trees_up(row, col);
                let down = self.visible_trees_down(row, col);
                let left = self.visible_trees_left(row, col);
                let right = self.visible_trees_right(row, col);

                let score = up * down * left * right;

                if score > highest_score {
                    highest_score = score
                }
            }
        }

        highest_score
    }
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect(&format!("Error reading the {} file.", file));

    let forest = Forest::from_str(contents);
    let highest_score = forest.find_highest_scenic_score();
    println!("Highest score: {}", highest_score);
}
