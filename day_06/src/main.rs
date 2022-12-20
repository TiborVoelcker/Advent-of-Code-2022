use std::collections::HashSet;

struct WindowIter<T> {
    size: usize,
    buffer: Vec<T>,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: std::clone::Clone> Iterator for WindowIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => self.buffer.push(e),
            None => return None,
        }
        if self.buffer.len() > self.size {
            self.buffer.remove(0);
        }

        Some(self.buffer.to_vec())
    }
}

fn main() {
    let contents = include_str!("../input.txt");

    let size = 14;

    let window_iter = WindowIter {
        size,
        iter: Box::new(contents.chars()),
        buffer: vec![],
    };

    for (i, window) in window_iter.enumerate().skip(size - 1) {
        let uniques: HashSet<char> = HashSet::from_iter(window.clone());

        println!("{window:?}");

        if uniques.len() == size {
            println!("Found {window:?} at marker {}.", i + 1);
            break;
        }
    }
}
