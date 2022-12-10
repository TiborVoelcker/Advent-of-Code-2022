use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

struct Elves {
    buffer: Lines<BufReader<File>>,
    last_idx: i32,
}

#[derive(Debug)]
struct Elf {
    idx: i32,
    calories: i32,
}

impl Iterator for Elves {
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {
        let mut calories = 0;
        while let Some(amount) = self.buffer.next() {
            let amount = amount.unwrap();

            if amount.is_empty() {
                self.last_idx += 1;
                return Some(Elf {
                    idx: self.last_idx,
                    calories,
                });
            } else {
                calories += amount.parse::<i32>().unwrap();
            }
        }
        if calories > 0 {
            return Some(Elf {
                idx: self.last_idx + 1,
                calories,
            });
        }
        None
    }
}

fn elves(file_path: String) -> Result<Elves, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(Elves {
        buffer: reader.lines(),
        last_idx: 0,
    })
}

fn main() {
    let file_path = env::args().nth(1).expect("File path missing.");

    let elves = elves(file_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    let mut elves: Vec<Elf> = elves.collect();
    elves.sort_by_key(|e| e.calories);

    println!(
        "The top elf has {} calories.",
        elves[elves.len() - 1].calories
    );

    println!(
        "The top 3 elves have {:?} calories.",
        &elves[elves.len() - 3..]
            .into_iter()
            .map(|e| e.calories)
            .sum::<i32>()
    )
}
