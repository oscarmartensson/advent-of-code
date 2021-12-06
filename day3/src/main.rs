use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn first_part(input_path: &str) {

    let mut nr_bits_vector: Vec<i64> = Vec::new();

    // Read file.
    if let Ok(lines) = read_lines(input_path) {

        let bits = Vec::from_iter(lines);
        let nr_bits = bits[0].as_ref().unwrap().len();
        nr_bits_vector.resize_with(nr_bits, Default::default);

        for i in 0..bits.len() as i64 {
            // Parse the binary numbers on the line.
            for (j, value) in bits[i as usize].as_ref().unwrap().chars().enumerate() {
                if value.to_digit(10) > Some(0) {
                    nr_bits_vector[j] += 1;
                } else {
                    nr_bits_vector[j] -= 1;
                }
            }
        }
        

        let mut gamma = 0;
        let mut epsilon = 0;
        // The value convertion from binary to decimal.
        let mut base = 1;
        // Parse the result from right->left (binary order)
        // and calculate the gamma resp. epsilon values.
        for i in nr_bits_vector.iter().rev() {
            // Dereference the pointer.
            if *i > 0 { gamma += base; }
            else      { epsilon += base; }
            base *= 2;
        }

        println!("epsilon * gamma: {}", epsilon * gamma);
    }
}

// fn to_decimal(binary_str: &String) -> u64 {

//     let mut decimal: u64 = 0;
//     for (i, bit) in binary_str.chars().rev().enumerate() {
//         let bit_num:u32 = bit.to_digit(10)
//         .expect("Error while parsing bit");
//         decimal += (bit_num * u32::pow(2, i as u32)) as u64;
//     }

//     return decimal;
// }

fn main() {
    first_part("input.txt");
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
