use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn dive(filename: &str) {
    let mut horiz = 0;
    let mut depth = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line.unwrap().split_once(" ") {
            Some(("up", value)) => depth = depth - value.parse::<i32>().unwrap(),
            Some(("down", value)) => depth = depth + value.parse::<i32>().unwrap(),
            Some(("forward", value)) => horiz = horiz + value.parse::<i32>().unwrap(),
            _ => continue,
        }
    }

    println!("{}", horiz * depth);
}

fn aim(filename: &str){
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line.unwrap().split_once(" ") {
            Some(("up", value)) => aim = aim - value.parse::<i32>().unwrap(),
            Some(("down", value)) => aim = aim + value.parse::<i32>().unwrap(),
            Some(("forward", value)) => {
                horiz = horiz + value.parse::<i32>().unwrap();
                depth += aim * value.parse::<i32>().unwrap();
            }
            _ => continue,
        }
    }

    println!("{}", depth * horiz);
}

fn main() {
    dive("day2.txt");
    aim("day2.txt")
}
