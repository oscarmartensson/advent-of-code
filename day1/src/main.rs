use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "input.txt";
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String

        let mut counter = 0u16;
        let mut previous_value:i64 = 0xDEADBEEF;
        let mut current_value:i64;
        for line in lines {
            if let Ok(ip) = line {

                current_value = ip.parse::<i64>().unwrap();
                if previous_value == 0xDEADBEEF {
                    // First iteration.
                    previous_value = current_value;
                    continue;
                }

                if current_value > previous_value{
                    counter += 1;
                }

                previous_value = current_value;
            }
        }

        println!("Number of times values decreased: {}", counter);
    }
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
