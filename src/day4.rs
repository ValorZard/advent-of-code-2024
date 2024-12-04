use std::fmt::Display;

const XMAS_STR: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX_STR: [char; 4] = ['S', 'A', 'M', 'X'];

fn check_horizontal(line: &Vec<char>, x: usize,) -> i32 {
    for i in 0..XMAS_STR.len() {
        // make sure the rest of the line makes up the rest of XMAS
        if x + i < line.len() {
            if line[x + i] != XMAS_STR[i] {
                return 0;
            }
        }
        else {
            return 0;
        }
    }
    println!("Found XMAS");
    1
}

fn check_reversed_horizontal(line: &Vec<char>, x: usize,) -> i32 {
    for i in 0..SAMX_STR.len() {
        // make sure the rest of the line makes up the rest of XMAS
        if x + i < line.len() {
            if line[x + i] != SAMX_STR[i] {
                return 0;
            }
        }
        else {
            return 0;
        }
    }
    println!("Found SAMX");
    1
}

fn check_vertical(lines: &Vec<&str>, x: usize, y: usize) -> i32 {
    for i in 0..XMAS_STR.len() {
        // make sure the rest of the line makes up the rest of XMAS
        if y + i < lines.len() {
            if lines[y + i].chars().nth(x).unwrap() != XMAS_STR[i] {
                return 0;
            }
        }
        else {
            return 0;
        }
    }
    println!("Found XMAS");
    1
}

fn check_reversed_vertical(lines: &Vec<&str>, x: usize, y: usize) -> i32 {
    for i in 0..SAMX_STR.len() {
        // make sure the rest of the line makes up the rest of XMAS
        if y + i < lines.len() {
            if lines[y + i].chars().nth(x).unwrap() != SAMX_STR[i] {
                return 0;
            }
        }
        else {
            return 0;
        }
    }
    println!("Found XMAS");
    1
}

fn run_xmas_checker(lines: &Vec<&str>, is_reversed: bool, x: usize, y: usize) -> i32 {
    //println!("Checking xmas at {}, {} -> {}", x, y, lines[y].chars().nth(x).unwrap());
    if is_reversed {
        //return check_reversed_horizontal(&lines[y].chars().collect(), x);
        return check_reversed_vertical(lines, x, y);
    }
    else {
        //return check_horizontal(&lines[y].chars().collect(), x);
        return check_vertical(lines, x, y);
    }
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
                total_xmas += run_xmas_checker(&lines, false, x, y);
            }
            else if c == XMAS_STR[3] {
                total_xmas += run_xmas_checker(&lines, true, x, y);
            }
        }
    }
    total_xmas
}

pub fn run_day_4_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}