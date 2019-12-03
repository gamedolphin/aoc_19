use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Pos(i32,i32);

fn get_x (d: char) -> i32 {
    match d {
        'L' => { -1 }
        'R' => { 1 }
        _ => { 0 }
    }
}

fn get_y (d: char) -> i32 {
    match d {
        'U' => {  1 }
        'D' => { -1 }
        _ => { 0 }
    }
}

fn str_from_pos (p: Pos) -> String {
    format!("{},{}",p.0,p.1)
}

fn pos_from_str (str: String) -> Pos {
    let p:Vec<&str> = str.split(',').collect();
    Pos(p[0].parse::<i32>().unwrap(),p[1].parse::<i32>().unwrap())
}

pub fn run() {
    let filename = "./info/day3/puzzle_input.txt";
    let reader = BufReader::new(File::open(filename).expect("Could not find file day3.txt"));
    let mut paths = Vec::new();
    for line in reader.lines() {
        let mut pos = Pos(0,0);
        let mut path = HashMap::new();
        let mut length = 0;
        for instruction in line.unwrap().split(',') {
            let dir = instruction.chars().nth(0).unwrap();
            let distance = &instruction[1..].parse::<i32>().unwrap();
            for _ in 0..*distance {
                pos.0 += get_x(dir);
                pos.1 += get_y(dir);
                path.entry(str_from_pos(Pos(pos.0, pos.1))).or_insert(length);
                length += 1;
            }
        }
        paths.push(path);
    }

    let path1 = paths.get(0).unwrap();
    let path2 = paths.get(1).unwrap();

    let mut all_paths = HashMap::new();
    // count the traversals
    for key in path1.keys() {
        *all_paths.entry(key).or_insert(0) += 1;
    }

    for key in path2.keys() {
        *all_paths.entry(key).or_insert(0) += 1;
    }

    let common = all_paths.iter().filter(|&(_,v)| *v > 1).map(|(k,_)| k);

    let mut min_distance = std::i32::MAX;
    let mut min_length = std::i32::MAX;
    for x in common {
        let pos = pos_from_str(x.to_string());
        let distance = pos.0.abs() + pos.1.abs();
        if distance < min_distance {
            min_distance = distance;
        }

        let path1_length = path1.get(&x.to_string()).unwrap() + 1;
        let path2_length = path2.get(&x.to_string()).unwrap() + 1;
        let signal_distance = path1_length + path2_length;

        if signal_distance < min_length {
            min_length = signal_distance;
        }
    }

    crate::day_info::print_info(3,1);
    crate::day_info::print_answer(format!("{}", min_distance));
    crate::day_info::print_info(3,2);
    crate::day_info::print_answer(format!("{}", min_length));
}
