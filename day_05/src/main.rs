use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl Move {
    fn from_str(line: String) -> Self {
        let binding = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");
        let parts = binding.trim().split(" ").collect::<Vec<&str>>();
        Self {
            from: parts[1].parse().unwrap(),
            to: parts[2].parse().unwrap(),
            count: parts[0].parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Stack {
    idx: i32,
    crates: Vec<char>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn from_str(mut lines: Vec<&String>) -> Self {
        let mut stacks = vec![];
        for idx in lines.pop().unwrap().trim().split("   ") {
            stacks.push(Stack {
                idx: idx.parse().unwrap(),
                crates: vec![],
            })
        }
        lines.reverse();

        for line in lines {
            for i in 0..stacks.len() {
                let c = line.chars().nth(i * 4 + 1).unwrap();
                if c != ' ' {
                    stacks[i].crates.push(c)
                }
            }
        }

        Self(stacks)
    }

    fn do_move(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let to_move = self.0[mv.from - 1].crates.pop().unwrap();
            self.0[mv.to - 1].crates.push(to_move);
        }
    }

    fn do_move2(&mut self, mv: &Move) {
        let from = &mut self.0[mv.from - 1].crates;
        let mut to_move = from.split_off(from.len() - mv.count).to_vec();

        self.0[mv.to - 1].crates.append(&mut to_move);
    }

    fn get_top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.crates.last().unwrap())
            .join("")
    }
}

fn read_lines(file_path: String) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn main() {
    let file_path = env::args().nth(1).expect("File path missing.");

    let mut lines = read_lines(file_path)
        .unwrap_or_else(|err| {
            eprintln!("Problem opening file: {err}");
            std::process::exit(1);
        })
        .map(|line| line.unwrap());

    // get starting position
    let init: Vec<String> = lines.by_ref().take_while(|line| line != "").collect();
    let mut stacks = Stacks::from_str(init.iter().collect());

    for line in lines {
        let mv = Move::from_str(line);
        stacks.do_move2(&mv);
    }

    println!("Arrangement: {}", stacks.get_top_crates());
}
