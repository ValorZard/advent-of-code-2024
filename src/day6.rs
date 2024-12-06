use std::fmt::Display;

pub fn run_day_6_part_1(contents: &str) -> impl Display + use<'_> {
    // convert to vec of byte array
    let mut guard_map = contents.lines().map(|x| x.as_bytes().to_vec()).rev().collect::<Vec<Vec<u8>>>();
    for y in 0..guard_map.len() {
       for x in 0..guard_map[y].len() {
            if char::from(guard_map[y][x]) == '^' {
                if y + 1 < guard_map.len() && guard_map[y+1][x] != b'#' as u8 {
                    guard_map[y+1][x] = '^' as u8;
                    guard_map[y][x] = 'X' as u8;
                }
            }
           print!("{}", guard_map[y][x] as char);
       }
    }
    contents
}

pub fn run_day_6_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}