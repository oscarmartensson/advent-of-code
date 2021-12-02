use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn first_part(input_path: &str) {

    let mut horizontal_distance: i64 = 0;
    let mut vertical_distance: i64 = 0;

    // Read file.
    if let Ok(lines) = read_lines(input_path) {

        for line in lines {
            if let Ok(ip) = line {

                let split_line = ip.split(' ').collect::<Vec<&str>>();
                let direction = split_line[0];
                let distance = split_line[1].parse::<i64>().unwrap() as i64;
                match direction {
                    "forward" => { horizontal_distance += distance; }
                    "up" => { vertical_distance -= distance; }
                    "down" => { vertical_distance += distance; }
                    _ => {/*All other cases, must be covered due to compilator E0004 complaint.*/}
                }
            }
        }

        println!("Final horizontal value: {}", horizontal_distance);
        println!("Final vertical value: {}", vertical_distance);
        println!("Multiplied by each other: {}", vertical_distance * horizontal_distance);
    }
}

fn second_part(input_path: &str) {

    let mut horizontal_distance: i64 = 0;
    let mut vertical_distance: i64 = 0;
    let mut aim = 0;

    // Read file.
    if let Ok(lines) = read_lines(input_path) {

        for line in lines {
            if let Ok(ip) = line {

                let split_line = ip.split(' ').collect::<Vec<&str>>();
                let direction = split_line[0];
                let distance = split_line[1].parse::<i64>().unwrap() as i64;

                match direction {
                    "forward" => {
                        horizontal_distance += distance;
                        vertical_distance += aim * distance;
                    }
                    "up" => { aim -= distance; }
                    "down" => { aim += distance; }
                    _ => {/*All other cases, must be covered due to compilator E0004 complaint.*/}
                }
            }
        }

        println!("Final horizontal value: {}", horizontal_distance);
        println!("Final vertical value: {}", vertical_distance);
        println!("Final aim value: {}", aim);
        println!("Multiplied by each other: {}", vertical_distance * horizontal_distance);
    }
}

fn main() {
    first_part("input.txt");
    second_part("input.txt");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error occured. Message: {}. Closing program.", err);
            std::process::exit(1);
        }
    };
    let iterator = io::BufReader::new(file).lines();

    return Ok(iterator);
}
