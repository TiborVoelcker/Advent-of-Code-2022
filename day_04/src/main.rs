use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Sections {
    from: i32,
    to: i32,
}

impl Sections {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split("-");
        Self {
            from: parts.next().unwrap().parse().unwrap(),
            to: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn contains(&self, other: Sections) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: Sections) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pair {
    first: Sections,
    second: Sections,
}

impl Pair {
    fn from_str(s: String) -> Self {
        let mut parts = s.split(",");
        Self {
            first: Sections::from_str(parts.next().unwrap()),
            second: Sections::from_str(parts.next().unwrap()),
        }
    }

    fn is_fully_overlapping(&self) -> bool {
        self.first.contains(self.second) || self.second.contains(self.first)
    }

    fn is_overlapping(&self) -> bool {
        self.first.overlaps(self.second) || self.second.overlaps(self.first)
    }
}

fn read_lines(file_path: String) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn main() {
    let a = 2;
    let file_path = env::args().nth(1).expect("File path missing.");

    let lines = read_lines(file_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {err}");
        std::process::exit(1);
    });

    let mut fully_overlapping = 0;
    let mut overlapping = 0;
    for line in lines {
        let pair = Pair::from_str(line.unwrap());
        if pair.is_fully_overlapping() {
            println!("Found fully overlapping pair: {pair:?}");
            fully_overlapping += 1;
        }
        if pair.is_overlapping() {
            println!("Found overlapping pair: {pair:?}");
            overlapping += 1;
        }
    }

    println!("Total fully overlapping: {fully_overlapping}.");
    println!("Total overlapping: {overlapping}.");
}
