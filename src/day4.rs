use std::fmt::Display;

const XMAS_STR: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn run_day_4_part_1(contents: &str) -> impl Display + use<'_> {
    let lines = contents.lines();
    let mut total_xmas = 0;
    //let mut possible_xmas : Vec<Vec<char>> = Vec::new();
    'outer: for line in contents.lines() {
        println!("{}", line);
        for (i, c) in line.chars().enumerate() {
            if XMAS_STR.contains(&c) {
                total_xmas += 1;
                continue 'outer;
            }
        }
    }
    total_xmas
}

pub fn run_day_4_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}