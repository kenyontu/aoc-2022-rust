#[derive(PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Value \"{value}\" has no corresponding hand."),
        }
    }
}

impl Hand {
    pub fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn match_against(&self, other: &Self) -> GameOutcome {
        use Hand::*;

        if self == other {
            GameOutcome::Draw
        } else if (self == &Rock && other == &Scissors)
            || (self == &Scissors && other == &Paper)
            || (self == &Paper && other == &Rock)
        {
            GameOutcome::Win
        } else {
            GameOutcome::Lose
        }
    }
}

enum GameOutcome {
    Win,
    Draw,
    Lose,
}

impl GameOutcome {
    pub fn points(&self) -> u32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Draw => 3,
            _ => 0,
        }
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (Hand::from(a), Hand::from(b)))
        .map(|(oponnent_hand, your_hand)| {
            let outcome = your_hand.match_against(&oponnent_hand);
            your_hand.points() + outcome.points()
        })
        .sum()
}
