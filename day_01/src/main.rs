use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args.get(1).expect("File path missing.");
    let file = File::open(file_path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let mut highest3 = [0, 0, 0];
    let mut current = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match line.parse::<i32>() {
            Ok(cal) => current += cal,
            _ => {
                if line.is_empty() {
                    if current >= highest3[0] {
                        let mut temp = [current, highest3[0], highest3[1], highest3[2]];
                        temp.sort();
                        highest3 = [temp[1], temp[2], temp[3]];
                    }
                    current = 0;
                }
            }
        }
    }

    println!("{:?}", highest3);
    let sum: i64 = (highest3[0] + highest3[1] + highest3[2]).into();
    println!("{}", sum);
}
