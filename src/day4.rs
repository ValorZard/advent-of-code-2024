use std::fmt::Display;

const XMAS_STR: [char; 4] = ['X', 'M', 'A', 'S'];

fn run_xmas_checker(lines: &Vec<&str>, is_reversed: bool, x: usize, y: usize) -> i32 {
    println!("Checking xmas at {}, {} -> {}", x, y, lines[y].chars().nth(x).unwrap());
    0
}

pub fn run_day_4_part_1(contents: &str) -> impl Display + use<'_> {
    let lines : Vec<&str> = contents.lines().collect();
    let mut total_xmas = 0;
    //let mut possible_xmas : Vec<Vec<char>> = Vec::new();
    for y in 0..lines.len() {
        let line = lines[y];
        println!("{}", line);
        for (x, c) in line.chars().enumerate() {
            if c == XMAS_STR[0] {
                run_xmas_checker(&lines, false, x, y);
            }
            else if c == XMAS_STR[3] {
                run_xmas_checker(&lines, true, x, y);
            }
        }
    }
    total_xmas
}

pub fn run_day_4_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}