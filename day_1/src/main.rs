use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = fs::File::open("input.txt").expect("No such file");
    let buf = BufReader::new(file);
    let collection: Vec<i16> = buf
        .lines()
        .map(|l| l.expect("could not parse").parse::<i16>().unwrap())
        .collect();

    let mut counter = 0;
    for i in 1..collection.len() {
        if collection[i] > collection[i - 1] {
            counter += 1;
        }
    }
    println!("Part 1: {}", counter);
}
