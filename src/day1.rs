use std::fs::File;
use std::io::{BufRead, BufReader};


fn calculate_rec(mass: i32) -> i32 {
    let sum = (mass / 3) - 2;
    if sum <= 0 { 0 } else { sum + calculate_rec(sum) }
}

pub fn run() {
    crate::day_info::print_info(1,1);
    let filename = "./info/day1/puzzle_input.txt";
    let mut sum = 0;
    let mut total_sum = 0;
    let reader = BufReader::new(File::open(filename).expect("Could not find file day1.txt"));
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            let val = word.parse::<i32>().unwrap();
            let fuel = (val / 3) - 2;
            sum += fuel;
            total_sum += calculate_rec(val);
        }
    }

    println!("Your puzzle answer was {}.", sum);

    crate::day_info::print_info(1,2);

    println!("Your puzzle answer was {}.", total_sum);
}
