use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Arrange
    let file = fs::File::open("input.txt").expect("No such file");
    let buf = BufReader::new(file);
    let collection: Vec<String> = buf.lines().map(|l| l.expect("could not parse")).collect();
    let mut counter: Vec<[u16; 2]> = Vec::new();
    // Part 1
    let first_row = &collection[0];
    for c in first_row.chars() {
        counter.push([if c == '0' { 1 } else { 0 }, if c == '1' { 1 } else { 0 }]);
    }
    for i in 1..collection.len() {
        for (i, c) in collection[i].chars().enumerate() {
            if c == '0' {
                counter[i][0] += 1;
            } else {
                counter[i][1] += 1;
            }
        }
    }
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    for arr in counter {
        if arr[0] > arr[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        };
    }
    let gamma_decimal = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = usize::from_str_radix(&epsilon, 2).unwrap();

    println!(
        "Part 1: {} (Gamma: {} = {}, Epsilon: {} = {})",
        gamma_decimal * epsilon_decimal,
        gamma,
        gamma_decimal,
        epsilon,
        epsilon_decimal
    );
 
    // Part 2
    let mut filtered_collection: Vec<String> = collection.clone();
    let mut ogr: String = "".to_string();
    for i in 0..collection[0].len() {
        let mut count = 0;
        filtered_collection = filtered_collection
            .iter()
            .filter(|row| row.starts_with(&ogr))
            .map(|row| row.to_string())
            .collect();

            if filtered_collection.len() == 1 {
                ogr = filtered_collection[0].to_string();
                break;
            }
            
            filtered_collection.iter().for_each(|row| {
            if row.chars().nth(i).unwrap() == '1' {
                count += 1
            }
        });
        let half: f32 = filtered_collection.len() as f32 / 2.0;
        if count as f32 >= half {
            ogr.push('1');
        } else {
            ogr.push('0')
        }
    }
    
    let mut filtered_collection: Vec<String> = collection.clone();
    let mut cosr: String = "".to_string();
    for i in 0..collection[0].len() {
        let mut count = 0;
        filtered_collection = filtered_collection
            .iter()
            .filter(|row| row.starts_with(&cosr))
            .map(|row| row.to_string())
            .collect();
        
        if filtered_collection.len() == 1 {
            cosr = filtered_collection[0].to_string();
            break;
        }
        
        filtered_collection.iter().for_each(|row| {
            if row.chars().nth(i).unwrap() == '0' {
                count += 1
            }
        });
        
        let half: f32 = filtered_collection.len() as f32 / 2.0;
        if count as f32 <= half {
            cosr.push('0');
        } else {
            cosr.push('1')
        }
    }
    
    let ogr_dec = usize::from_str_radix(&ogr, 2).unwrap();
    let cosr_dec = usize::from_str_radix(&cosr, 2).unwrap();
    
    let answer = usize::from_str_radix(&ogr, 2).unwrap() * usize::from_str_radix(&cosr, 2).unwrap();
    println!(
        "Part 2: {} (OGR: {} = {}, COSR: {} = {})",
        answer, ogr, ogr_dec, cosr, cosr_dec
    );
}
