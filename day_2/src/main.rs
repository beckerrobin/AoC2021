use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Arrange
    let file = fs::File::open("input.txt").expect("No such file");
    let buf = BufReader::new(file);
    let collection: Vec<String> = buf.lines().map(|l| l.expect("could not parse")).collect();

    let mut horizontal:u32 = 0;
    let mut depth:u32 = 0;

    // Part 1
    for i in 0..collection.len() {
        //println!("{}", collection[i].find(char::is_whitespace).unwrap());
        let (command, n): (&str, &str) = collection[i].split_at(collection[i].find(char::is_whitespace).unwrap());
        let amount = n.trim_start().parse::<u32>().unwrap();
        match command {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!(),
        }
    }
    println!("Part 1: {}", horizontal * depth);
}
