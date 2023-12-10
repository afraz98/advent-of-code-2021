use std::fs::read_to_string;

fn binary_diagnostic(filename: &str){
    let mut lines = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string())
    }

    let binary_length = lines[0].len();
    let number_binaries = lines.len();
    let mut digit_counts = vec![0 ; binary_length];

    for line in lines {
        for (index, digit)  in line.chars().enumerate() {
            match digit {
                '1' => { digit_counts[index] += 1 },
                _ => { continue },
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for count in digit_counts {
        if count > (number_binaries - count) { 
            // Most common digit is a '1'
            gamma.push('1');
            epsilon.push('0');
        } else {
            // Most common digit is a '0'
            gamma.push('0');
            epsilon.push('1');
        }
    }

    // Adapted from here https://stackoverflow.com/questions/27606616/is-there-anything-in-rust-to-convert-a-binary-string-to-an-integer
    let gamma_base10 = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_base10 = isize::from_str_radix(&epsilon, 2).unwrap();
    
    println!("{}", gamma_base10 * epsilon_base10);
}

fn main() { binary_diagnostic("day3.txt") }
