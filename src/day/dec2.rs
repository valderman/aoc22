#[derive(PartialEq, Debug)]
pub enum Hand { Rock, Paper, Scissors }

#[derive(PartialEq, Debug)]
pub enum Outcome { Win, Draw, Lose }

impl Outcome {
    pub fn parse(c: char) -> Option<Outcome> {
        match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl Hand {
    pub fn parse(c: char) -> Option<Hand> {
        match c {
            'A' | 'X' => Some(Hand::Rock),
            'B' | 'Y' => Some(Hand::Paper),
            'C' | 'Z' => Some(Hand::Scissors),
            _ => None
        }
    }
    
    pub fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn play(&self, other_hand: &Hand) -> Outcome {
        if *self == *other_hand {
            return Outcome::Draw
        }
        match self {
            Hand::Rock if *other_hand == Hand::Scissors => Outcome::Win,
            Hand::Paper if *other_hand == Hand::Rock => Outcome::Win,
            Hand::Scissors if *other_hand == Hand::Paper => Outcome::Win,
            _ => Outcome::Lose
        }
    }
}

pub fn parse_move<T, R>(
    str: &String,
    parse_second_part: fn(char) -> Option<T>,
    make_move: fn(Hand, T) -> R
) -> Option<R> {
    let mut chars = str.chars();
    let first = Hand::parse(chars.next()?)?;
    if chars.next() != Some(' ') {
        return None
    }
    let second = parse_second_part(chars.next()?)?;
    Some(make_move(first, second))
}
