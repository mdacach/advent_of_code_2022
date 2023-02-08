use std::str::FromStr;

use itertools::{process_results, Itertools};

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move_against(self) -> Move {
        Move::ALL_MOVES
            .iter()
            .copied()
            .find(|m| Round::beats(*m, self))
            .unwrap()
    }

    fn losing_move_against(self) -> Move {
        Move::ALL_MOVES
            .iter()
            .copied()
            .find(|m| Round::beats(self, *m))
            .unwrap()
    }

    fn drawing_move_against(self) -> Move {
        self
    }

    fn points(self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Round {
    opponent: Move,
    player: Move,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn points(self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, opponent: Move) -> Move {
        match self {
            Outcome::Win => opponent.winning_move_against(),
            Outcome::Draw => opponent.drawing_move_against(),
            Outcome::Loss => opponent.losing_move_against(),
        }
    }
}

impl Round {
    fn beats(left: Move, right: Move) -> bool {
        matches!(
            (left, right),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn outcome(self) -> Outcome {
        if Round::beats(self.player, self.opponent) {
            Outcome::Win
        } else if Round::beats(self.opponent, self.player) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn get_score(self) -> u64 {
        self.player.points() + self.outcome().points()
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let (opponent, outcome) = (chars[0], chars[2]);
        let opponent = Move::try_from(opponent)?;
        let outcome = Outcome::try_from(outcome)?;
        let player_move = outcome.matching_move(opponent);

        Ok(Round {
            opponent,
            player: player_move,
        })
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("../input.txt");
    let total_score: u64 = process_results(
        lines
            .lines()
            .map(Round::from_str)
            .map_ok(|round| round.get_score()),
        |iter| iter.sum(),
    )
    .unwrap();

    println!("{total_score}");
    Ok(())
}
