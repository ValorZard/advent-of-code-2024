use std::fmt::Display;

const XMAS_STR: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX_STR: [char; 4] = ['S', 'A', 'M', 'X'];

fn find_word(grid: &[&[u8]], word: &[u8], row: usize, col: usize, dr: isize, dc: isize) -> u32 {
    if grid[row][col] == word[0] {
        if word.len() == 1 {
            return 1;
        }

        let Some(r) = row.checked_add_signed(dr) else {
            return 0;
        };
        let Some(c) = col.checked_add_signed(dc) else {
            return 0;
        };
        if r >= grid.len() || c >= grid[r].len() {
            return 0;
        }

        return find_word(grid, &word[1..], r, c, dr, dc);
    }

    0
}

fn run_xmas_checker(lines: &[&[u8]], is_reversed: bool, x: usize, y: usize) -> u32 {
    //println!("Checking xmas at {}, {} -> {}", x, y, lines[y].chars().nth(x).unwrap());
    if is_reversed {
        return find_word(lines, "SAMX".as_bytes(), x, y, 1, 0) + find_word(lines, "SAMX".as_bytes(), x, y, 0, 1) + find_word(lines, "SAMX".as_bytes(), x, y, 1, 1) + find_word(lines, "SAMX".as_bytes(), x, y, -1, -1);
    }
    else {
        return find_word(lines, "XMAS".as_bytes(), x, y, 1, 0) + find_word(lines, "XMAS".as_bytes(), x, y, 0, 1) + find_word(lines, "XMAS".as_bytes(), x, y, 1, 1) + find_word(lines, "XMAS".as_bytes(), x, y, -1, -1);
    }
}

pub fn run_day_4_part_1(contents: &str) -> impl Display + use<'_> {
    let lines : Vec<&str> = contents.lines().collect();
    let lines : Vec<&[u8]> = lines.iter().map(|x| x.as_bytes()).collect();
    let mut total_xmas = 0;
    //let mut possible_xmas : Vec<Vec<char>> = Vec::new();
    for x in 0..lines.len() {
        let line = lines[x];
        for y in 0..line.len() {
            total_xmas += run_xmas_checker(&lines, false, x, y) + run_xmas_checker(&lines, true, x, y);
        }
    }
    total_xmas
}

pub fn run_day_4_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}