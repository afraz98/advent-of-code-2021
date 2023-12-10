use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string())
    }

    lines
}


fn get_digit_counts(lines: Vec<String>, line_length: usize) -> Vec<usize> {
    let mut digit_counts = vec![0 ; line_length];
    for line in lines {
        for (index, digit)  in line.chars().enumerate() {
            match digit {
                '1' => { digit_counts[index] += 1 },
                _ => { continue },
            }
        }
    } 

    return digit_counts
}


fn solve_part_one(filename: &str){
    let lines = parse_input(filename);

    let binary_length = lines[0].len();
    let number_binaries = lines.len();
    let digit_counts = get_digit_counts(lines, binary_length);

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

fn filter_binaries(lines: Vec<String>, index: usize, comparator: char) -> Vec<String> {
    return lines.into_iter().filter(|s| s.chars().nth(index).unwrap() == comparator).collect::<Vec<String>>();
}

fn solve_part_two(filename: &str){
    let mut filtered_lines: Vec<String> = parse_input(filename);
    let mut l = filtered_lines.len();
    let mut index = 0;
    let binary_length = filtered_lines[0].len();
    
    while l > 1 {
        let digit_counts = get_digit_counts(filtered_lines.clone(), binary_length);
        
        if digit_counts[index] >= (l - digit_counts[index]) {
            filtered_lines = filter_binaries(filtered_lines, index, '1');
        } else {
            filtered_lines = filter_binaries(filtered_lines, index, '0');
        }
        
        index = index + 1;
        l = filtered_lines.len();
    }


    let o2_rating = isize::from_str_radix(&filtered_lines[0], 2).unwrap(); 

    let mut filtered_lines_two = parse_input(filename);
    l = filtered_lines_two.len();
    index = 0;

    while l > 1 {        
        let digit_counts = get_digit_counts(filtered_lines_two.clone(), binary_length);

        if (l - digit_counts[index]) <= digit_counts[index]  {
            filtered_lines_two = filter_binaries(filtered_lines_two, index, '0');
        } else {
            filtered_lines_two = filter_binaries(filtered_lines_two, index, '1');
        }

        index = index + 1;
        l = filtered_lines_two.len();
    }
    
    let co2_rating = isize::from_str_radix(&filtered_lines_two[0], 2).unwrap();

    println!("{}", co2_rating * o2_rating)
}

fn main() { 
    solve_part_one ("day3.txt"); 
    solve_part_two("day3.txt");
}
