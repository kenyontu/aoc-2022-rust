use std::fs;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_str(hand_str: &str) -> Result<Hand, String> {
        match hand_str {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(format!("Value \"{}\" has no corresponding hand.", hand_str)),
        }
    }

    fn get_points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn get_points_for_outcome(&self, outcome: &str) -> u32 {
        match outcome {
            "X" => match self {
                Hand::Rock => Hand::Scissors.get_points(),
                Hand::Paper => Hand::Rock.get_points(),
                Hand::Scissors => Hand::Paper.get_points(),
            },
            "Y" => match self {
                Hand::Rock => Hand::Rock.get_points() + 3,
                Hand::Paper => Hand::Paper.get_points() + 3,
                Hand::Scissors => Hand::Scissors.get_points() + 3,
            },
            "Z" => match self {
                Hand::Rock => Hand::Paper.get_points() + 6,
                Hand::Paper => Hand::Scissors.get_points() + 6,
                Hand::Scissors => Hand::Rock.get_points() + 6,
            },
            _ => 0,
        }
    }
}

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file);
    contents
}

fn main() {
    let contents = read_file("input.txt").expect("Error reading file");

    let mut total_score = 0;
    for line in contents.lines() {
        let game: Vec<&str> = line.split(' ').collect();
        if game.len() != 2 {
            continue;
        }

        let opponent_hand = Hand::from_str(game[0]).unwrap();
        let outcome = game[1];

        total_score += opponent_hand.get_points_for_outcome(&outcome);
    }

    println!("Total score is: {}", total_score);
}
