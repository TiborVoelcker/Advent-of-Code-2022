use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Choice {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissor,
            _ => panic!("Invalid character for Choice: {s}"),
        }
    }

    fn from_outcome(other: Choice, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Loose => other.beats(),
            Outcome::Draw => other,
            Outcome::Win => other.beats().beats(),
        }
    }

    fn beats(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid character for Choice: {s}"),
        }
    }

    fn from_choices(mine: Choice, other: Choice) -> Self {
        match mine {
            _ if mine.beats() == other => Self::Win,
            _ if other.beats() == mine => Self::Loose,
            other => Self::Draw,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Game {
    other: Choice,
    mine: Choice,
    outcome: Outcome,
}

impl Game {
    fn from_str1(s: String) -> Self {
        let mut parts = s.split(" ");
        let other = Choice::from_str(parts.next().unwrap());
        let mine = Choice::from_str(parts.next().unwrap());
        Self {
            other,
            mine,
            outcome: Outcome::from_choices(mine, other),
        }
    }

    fn from_str2(s: String) -> Self {
        let mut parts = s.split(" ");
        let other = Choice::from_str(parts.next().unwrap());
        let outcome = Outcome::from_str(parts.next().unwrap());
        Self {
            other,
            mine: Choice::from_outcome(other, outcome),
            outcome,
        }
    }

    fn score(&self) -> u32 {
        self.outcome as u32 + self.mine as u32
    }
}

fn read_lines(file_path: String) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn main() {
    let file_path = env::args().nth(1).expect("File path missing.");

    let lines = read_lines(file_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {err}");
        std::process::exit(1);
    });

    let mut total_score = 0;
    for line in lines {
        let game = Game::from_str2(line.unwrap());
        total_score += game.score();

        println!("{game:?} => {:?}", game.score());
    }

    println!("Total score: {total_score}.")
}
