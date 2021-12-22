use regex::Regex;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Arrange
    let file = fs::File::open("input.txt").expect("No such file ${cwd}");
    let buf = BufReader::new(file);
    let collection: Vec<String> = buf.lines().map(|l| l.expect("could not parse")).collect();

    // Part 1

    // Create vector of 2d bool matrices filled with false
    let mut matrices: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; 5]; 5]; (collection.len() - 1) / 6];
    
    let re = Regex::new(r"\s+").unwrap();

    /* For each number in the first row:
    * Go trough each 2d matrix (step 6 lines to next matrix)
    * check each number in each line and mark the position as true in the corresponding "boolean matrix" */
    for current_number in collection[0].split(",") {
        for arr in (2..collection.len()).step_by(6) {
            let matrix_i = (arr - 2) / 6;
            //println!("{}", matrix_i);
            for arr_row in 0..5 {
                let line: Vec<&str> = re.split(&collection[arr + arr_row].trim()).collect();
                let pos = line.iter().position(|&n| n == current_number);
                if pos != None {
                    // Got a number!
                    matrices[matrix_i][arr_row][pos.unwrap()] = true;

                    // See if this array has bingo; first horizontally else vertically.
                    if !matrices[matrix_i][arr_row].contains(&false) {
                        println!(
                            "Part 1: {}",
                            unmarked_sum(&collection, arr, &matrices[matrix_i]) * current_number.parse::<usize>().unwrap()
                        );
                        return;
                    } else {
                        for nth in 0..5 {
                            let vertical: Vec<bool> =
                                matrices[matrix_i].iter().map(|arr| arr[nth]).collect();
                            if !vertical.contains(&false) {
                                println!(
                                    "Part 1: {}",
                                    unmarked_sum(&collection, arr, &matrices[matrix_i]) * current_number.parse::<usize>().unwrap()
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn unmarked_sum(collection: &Vec<String>, i: usize, bool_matrix: &Vec<Vec<bool>>) -> usize {
    let mut unmarked_sum: u32 = 0;
    let re = Regex::new(r"\s+").unwrap();

    for row in i..i + 5 {
        for n in 0..5 {
            if !bool_matrix[row - i][n] {
                unmarked_sum += re.split(collection[row].trim()).collect::<Vec<&str>>()[n]
                    .parse::<u32>()
                    .unwrap();
            }
        }
    }
    
    return unmarked_sum.try_into().unwrap();
}
