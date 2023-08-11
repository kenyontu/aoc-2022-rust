use std::{collections::HashSet, fs};

use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: u32,
}

impl Step {
    fn new(direction: Direction, distance: u32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

struct Rope {
    knots: Vec<(i32, i32)>,
    pos_visited_by_tail: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        Self {
            knots: vec![(0, 0); knot_count],
            pos_visited_by_tail: HashSet::from([(0, 0)]),
        }
    }

    fn exec_step(&mut self, step: &Step) {
        let delta = match step.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..step.distance {
            self.knots[0] = (self.knots[0].0 + delta.0, self.knots[0].1 + delta.1);

            let mut i = 0;

            while i < self.knots.len() - 1 {
                if let Some(next_pos) = Self::get_next_pos(&self.knots[i], &self.knots[i + 1]) {
                    self.knots[i + 1] = next_pos;
                    i += 1;
                } else {
                    break;
                }
            }

            self.pos_visited_by_tail
                .insert(self.knots.last().unwrap().clone());
        }
    }

    /// Checks if the knots are far from each other
    fn are_knots_far(knot_one: &(i32, i32), knot_two: &(i32, i32)) -> bool {
        let y_dist = (knot_one.0 - knot_two.0).abs();
        let x_dist = (knot_one.1 - knot_two.1).abs();

        if y_dist == 1 && x_dist == 1 {
            return false;
        }

        y_dist >= 2 || x_dist >= 2
    }

    /// Calculates the next position of a knot relative to a target knot
    fn get_next_pos(target_knot: &(i32, i32), knot: &(i32, i32)) -> Option<(i32, i32)> {
        // If the knots are not far from each other, then no movement is required
        if !Self::are_knots_far(target_knot, knot) {
            return None;
        }

        // Same column
        if target_knot.0 == knot.0 {
            if target_knot.1 > knot.1 {
                return Some((knot.0, knot.1 + 1));
            } else {
                return Some((knot.0, knot.1 - 1));
            }
        }

        // Same row
        if target_knot.1 == knot.1 {
            if target_knot.0 > knot.0 {
                return Some((knot.0 + 1, knot.1));
            } else {
                return Some((knot.0 - 1, knot.1));
            }
        }

        // If not in the same row or column, it needs to move diagonally
        let movement = Self::get_diagonal_movement(target_knot, knot);
        Some((knot.0 + movement.0, knot.1 + movement.1))
    }

    /// Returns the diagonal movement of a knot relative to a target knot
    fn get_diagonal_movement(target_knot: &(i32, i32), knot: &(i32, i32)) -> (i32, i32) {
        if target_knot.0 > knot.0 && target_knot.1 > knot.1 {
            // Target is in the top-right
            (1, 1)
        } else if target_knot.0 > knot.0 && target_knot.1 < knot.1 {
            // Target is in the bottom-right
            (1, -1)
        } else if target_knot.0 < knot.0 && target_knot.1 < knot.1 {
            // Target is in the bottom-left
            (-1, -1)
        } else {
            // Target is in the top-left
            (-1, 1)
        }
    }
}

/// Parses the input string into a vector of [Step]s
fn parse_input(input: String) -> Vec<Step> {
    let step_regex = Regex::new(r"^(?<direction>[A-Z]) (?<distance>\d+)$").unwrap();

    let mut steps: Vec<Step> = Vec::new();
    for line in input.lines() {
        if let Some(captures) = step_regex.captures(line) {
            let direction_str = &captures["direction"];
            let distance_str = &captures["distance"];

            let direction = match direction_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = distance_str.parse::<u32>().unwrap();
            steps.push(Step::new(direction, distance));
        }
    }

    steps
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect(&format!("Error reading the {} file.", file));

    let steps = parse_input(contents);

    // For part 1 of the question, just pass a knot count of 2
    let mut rope = Rope::new(10);

    for step in steps.iter() {
        rope.exec_step(step);
    }

    println!("Visited count: {}", rope.pos_visited_by_tail.len());
}
