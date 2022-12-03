#[derive(Clone, Copy)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn play_against(&self, other: RockPaperScissors) -> RockPaperScissorsResult {
        use RockPaperScissors::*;
        use RockPaperScissorsResult::*;

        match (self, other) {
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            _ => Draw,
        }
    }

    pub fn result_against(&self, result: RockPaperScissorsResult) -> Self {
        use RockPaperScissors::*;
        use RockPaperScissorsResult::*;

        match (self, result) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Loss) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Loss) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Loss) => Paper,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }
}

pub enum RockPaperScissorsResult {
    Win,
    Draw,
    Loss,
}

impl RockPaperScissorsResult {
    pub fn score(&self) -> u32 {
        match self {
            RockPaperScissorsResult::Win => 6,
            RockPaperScissorsResult::Draw => 3,
            RockPaperScissorsResult::Loss => 0,
        }
    }
}
