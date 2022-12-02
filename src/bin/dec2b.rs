use aoc22::{io, day::dec2::{Hand, Outcome, parse_move}};

#[derive(Debug)]
struct Move(Hand, Outcome);

impl Move {
    fn score(&self) -> u32 {
        self.1.score() + self.needed_hand().score()
    }

    fn needed_hand(&self) -> Hand {
        match self.1 {
            _ if self.1 == Hand::Rock.play(&self.0) => Hand::Rock,
            _ if self.1 == Hand::Paper.play(&self.0) => Hand::Paper,
            _ => Hand::Scissors
        }
    }
}

fn main() {
    let moves = crate::io::safe_lines().map(|s|
        parse_move(&s, Outcome::parse, Move)
    );
    let score: u32 = moves.filter_map(|m| Some(m?.score())).sum();
    println!("{}", score)
}
