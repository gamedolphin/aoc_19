use std::fs;

fn run_program(mut vals: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut index:usize = 0;
    let count:usize = vals.len();

    vals[1] = noun;
    vals[2] = verb;

    while index < count - 4 {
        let opcode = vals[index];
        let add1 = vals[index + 1] as usize;
        let add2 = vals[index + 2] as usize;
        let add3 = vals[index + 3] as usize;

        if opcode == 99 {
            break;
        }

        let val1 = vals[add1];
        let val2 = vals[add2];

        if opcode == 1 {
            vals[add3] = val1 + val2;
        }

        if opcode == 2 {
            vals[add3] = val1 * val2;
        }

        index += 4;
    }

    vals[0]
}

pub fn run() {
    crate::day_info::print_info(2,1);

    let filename = "./info/day2/puzzle_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Unable to read day2.txt");

    let split = contents.split(',');
    let vals:Vec<i64> = split.map(|val| {
        val.trim().parse::<i64>().unwrap()
    }).collect();

    let part1 = run_program(vals.clone(), 12, 2);
    println!("Your puzzle answer was {}.", part1);

    crate::day_info::print_info(2,2);

    let comp_val = 19_690_720;

    for i in 1..99 {
        for j in 1..99 {
            let result = run_program(vals.clone(), i, j);
            if result == comp_val {
                let answer = (100*i)+j;
                println!("Your puzzle answer was {}.", answer);
                break;
            }
        }
    }
}
