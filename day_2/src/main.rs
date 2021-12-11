use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Arrange
    let file = fs::File::open("input.txt").expect("No such file");
    let buf = BufReader::new(file);
    let collection: Vec<String> = buf.lines().map(|l| l.expect("could not parse")).collect();
    let mut commands: Vec<(&str, u32)> = Vec::new();

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    // Part 1
    for i in 0..collection.len() {
        //println!("{}", collection[i].find(char::is_whitespace).unwrap());
        let (command, n): (&str, &str) =
            collection[i].split_at(collection[i].find(char::is_whitespace).unwrap());
        let amount = n.trim_start().parse::<u32>().unwrap();
        commands.push((command, amount));
        match command {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!(),
        }
    }
    println!("Part 1: {}", horizontal * depth);
    
    // Part 2
    let mut aim = 0;
    horizontal = 0;
    depth = 0;
    for i in 0..commands.len() {
        let (command, amount) = commands[i];
        match command {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!(),
        }
    }
    println!("Part 2: {}", horizontal * depth);
}
