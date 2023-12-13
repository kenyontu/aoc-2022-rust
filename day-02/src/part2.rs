#[derive(PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        use Hand::*;
        match value {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
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

    pub fn get_hand_for_outcome(&self, outcome: &GameOutcome) -> Self {
        use GameOutcome::*;
        use Hand::*;

        match self {
            Rock => match outcome {
                Win => Paper,
                Draw => Rock,
                Lose => Scissors,
            },
            Paper => match outcome {
                Win => Scissors,
                Draw => Paper,
                Lose => Rock,
            },
            Scissors => match outcome {
                Win => Rock,
                Draw => Scissors,
                Lose => Paper,
            },
        }
    }
}

#[derive(PartialEq, Eq)]
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

impl From<&str> for GameOutcome {
    fn from(value: &str) -> Self {
        use GameOutcome::*;
        match value {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Value \"{value}\" has no corresponding outcome."),
        }
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (Hand::from(a), GameOutcome::from(b)))
        .map(|(oponnent_hand, outcome)| {
            let your_hand = oponnent_hand.get_hand_for_outcome(&outcome);
            your_hand.points() + outcome.points()
        })
        .sum()
}
