use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "input.txt";

    // Read file.
    if let Ok(lines) = read_lines(file_name) {

        // --- Part 1 ---
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

    // Read the input again, easiest this way even though ugly.
    if let Ok(lines) = read_lines(file_name) {

        // --- Part 2 ---
        let mut counter = 0u16;
        let mut current_sum: i64;
        let mut previous_sum: i64 = 0xDEADBEEFi64;

        let mut moving_window: [i64; 3] = [0, 0, 0];
        let v = Vec::from_iter(lines);

        for i in 0..v.len() {
            if i + 2 < v.len()
            {
                moving_window[0] = v[i].as_ref().unwrap().parse::<i64>().unwrap();
                moving_window[1] = v[i+1].as_ref().unwrap().parse::<i64>().unwrap();
                moving_window[2] = v[i+2].as_ref().unwrap().parse::<i64>().unwrap();

                current_sum = moving_window[0] + moving_window[1] + moving_window[2];

                if previous_sum == 0xDEADBEEF {
                    // First iteration.
                    previous_sum = current_sum;
                    continue;
                }

                if current_sum > previous_sum {
                    counter += 1;
                }

                previous_sum = current_sum;
            }
        }

        println!("Number of times moving window sum increased: {}", counter);
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
