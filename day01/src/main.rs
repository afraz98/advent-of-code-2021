use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn parse_readings(filename: &str) -> Vec<i64>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let readings: Vec<i64> = reader.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect();
    return readings;
}

fn count_increasing_measurements(readings: Vec<i64>) {
    let mut count = 0;

    for i in 0..readings.len() - 1 {
        if readings[i+1] > readings[i] {
            count = count + 1
        }
    }
    println!("{}", count);
}



pub fn main(){
    // Part 1
    let readings = parse_readings("day1.txt");
    count_increasing_measurements(readings);

    // Part 2
    let readings = parse_readings("day1.txt");
    let mut sums: Vec<i64> = Vec::new();
    for i in 0..readings.len() - 2 {
        sums.push(readings[i]+readings[i+1]+readings[i+2])
    }

    count_increasing_measurements(sums);
}

