use aoc22::{io, day::dec2::{Hand, Outcome, parse_move}};

#[derive(Debug)]
struct Move(Hand, Hand);

impl Move {
    fn score(&self) -> u32 {
        self.1.score() + self.play().score()
    }

    fn play(&self) -> Outcome {
        self.1.play(&self.0)
    }
}

fn main() {
    let moves = crate::io::safe_lines().map(|s|
        parse_move(&s, Hand::parse, Move)
    );
    let score: u32 = moves.filter_map(|m| Some(m?.score())).sum();
    println!("{}", score)
}
