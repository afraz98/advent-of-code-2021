use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string())
    }

    lines
}

fn solve_part_one(filename: &str){
    let lines = parse_input(filename);
    for line in lines.clone() {
        println!("{}", line);
    }

    for number in lines[0].split(",") {
        println!("{}", number);
    }
}

fn main() {
    solve_part_one("day4.txt")
}
