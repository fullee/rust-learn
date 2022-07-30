use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn dispay(self) {
        for (key, value) in self.0.iter() {
            println!("{}:{}", key, value);
        }
    }
}

fn main() {
    let arguments = env::args().into_iter().;
    let filename = arguments;
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Cloud not open file");
    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();
    for line in reader.lines() {
        let line = line.expect("Cloud not read line");
        let words = line.split(" ");
        for word in words {
            if word == " " {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.dispay();
}

