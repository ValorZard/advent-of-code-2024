use std::fmt::Display;

pub fn run_day_6_part_1(contents: &str) -> impl Display + use<'_> {
    // convert to vec of byte array
    let mut guard_map = contents.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut finished = false;
    while !finished {
        for y in 0..guard_map.len() {
            for x in 0..guard_map[y].len() {
                    if char::from(guard_map[y][x]) == '^' {
                        if let Some(new_y) = y.checked_add_signed(-1) {
                            if guard_map[new_y][x] != b'#' as u8 {
                                guard_map[new_y][x] = '^' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                guard_map[y][x] = '>' as u8;
                            }
                        }
                        else {
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == '>' {
                        if x + 1 < guard_map[y].len() {
                            if guard_map[y][x + 1] != b'#' as u8 {
                                guard_map[y][x + 1] = '>' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                guard_map[y][x] = 'v' as u8;
                            }
                        }
                        else {
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == 'v' {
                        if y + 1 < guard_map[y].len() {
                            if guard_map[y + 1][x] != b'#' as u8 {
                                guard_map[y + 1][x] = 'v' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                guard_map[y][x] = '<' as u8;
                                finished = true;
                            }
                        }
                        else {
                            finished = true;
                        }
                    }
                print!("{}", guard_map[y][x] as char);
            }
            println!()
        }
    }
    println!("full map");
    for y in 0..guard_map.len() {
        for x in 0..guard_map[y].len() {
            print!("{}", guard_map[y][x] as char);
        }
        println!()
     }
    contents
}

pub fn run_day_6_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}