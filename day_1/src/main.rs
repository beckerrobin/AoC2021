use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Arrange
    let file = fs::File::open("input.txt").expect("No such file");
    let buf = BufReader::new(file);
    let collection: Vec<i16> = buf
        .lines()
        .map(|l| l.expect("could not parse").parse::<i16>().unwrap())
        .collect();

    // Part 1
    let mut counter = 0;
    for i in 1..collection.len() {
        if collection[i] > collection[i - 1] {
            counter += 1;
        }
    }
    println!("Part 1: {}", counter);
    
    // Part 2
    counter = 0;
    for i in 1..collection.len()-2 {
        if collection[i-1..i+2].iter().sum::<i16>() < collection[i..i+3].iter().sum::<i16>() {
            counter += 1;
        }
    }
    println!("Part 2: {}", counter);
}
