use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn read_lines(file_path: String) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn intersection(first: &str, second: &str) -> Vec<char> {
    let mut set1 = HashSet::new();
    for c in first.chars() {
        set1.insert(c);
    }
    let mut set2 = HashSet::new();
    for c in second.chars() {
        set2.insert(c);
    }
    set1.intersection(&set2).map(|c| *c).collect()
}

fn find_wrong_item(line: String) -> usize {
    let (container1, container2) = line.split_at(line.len() / 2);
    let c = *intersection(container1, container2)
        .iter()
        .next()
        .unwrap_or_else(|| panic!("No intersection with {container1} and {container2}."));

    CHARS.find(c).unwrap() + 1
}

fn find_group(group: Vec<String>) -> usize {
    let a = intersection(&group[0], &group[1]);
    let b = intersection(&a.iter().collect::<String>(), &group[2]);
    let c = *b
        .iter()
        .next()
        .unwrap_or_else(|| panic!("No intersection in {group:?}."));

    CHARS.find(c).unwrap() + 1
}

fn main() {
    let file_path = env::args().nth(1).expect("File path missing.");

    let lines = read_lines(file_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {err}");
        std::process::exit(1);
    });

    let mut total_score1 = 0;
    let mut total_score2 = 0;
    let mut group = vec![];
    for line in lines {
        let line = line.unwrap();
        group.push(line.to_string());

        if group.len() == 3 {
            total_score2 += find_group(group);
            group = vec![];
        }

        total_score1 += find_wrong_item(line);
    }

    println!("Total score part 1: {total_score1}");
    println!("Total score part 2: {total_score2}");
}
