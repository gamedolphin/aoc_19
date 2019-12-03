use std::fs;

pub fn print_info(day: u8, part: u8) {
    let p_1 = format!("./info/day{}/part{}.txt", day, part);
    let contents_1 = fs::read_to_string(p_1)
        .expect("Unable to read day2.txt");
    println!("{}",contents_1);
}

pub fn print_answer(answer: String) {
    println!("Your puzzle answer was {}.", answer);
    println!();
}
