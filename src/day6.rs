use std::{collections::HashSet, fmt::Display};

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
                            guard_map[y][x] = 'X' as u8;
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
                            guard_map[y][x] = 'X' as u8;
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
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == '<' {
                        if let Some(new_x) = x.checked_add_signed(-1) {
                            if guard_map[y][new_x] != b'#' as u8 {
                                guard_map[y][new_x] = '<' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                guard_map[y][x] = '^' as u8;
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                print!("{}", guard_map[y][x] as char);
            }
            println!();
        }
    }
    println!("full map");
    let mut count = 0;
    for y in 0..guard_map.len() {
        for x in 0..guard_map[y].len() {
            print!("{}", guard_map[y][x] as char);
            if guard_map[y][x] == 'X' as u8 {
                count += 1;
            }
        }
        println!()
     }
    count
}

fn loop_checker(guard_map: &mut Vec<Vec<u8>>) -> bool {
    let mut finished = false;
    let mut turn_points: HashSet<(usize, usize)> = HashSet::new();
    while !finished {
        for y in 0..guard_map.len() {
            for x in 0..guard_map[y].len() {
                    if char::from(guard_map[y][x]) == '^' {
                        if let Some(new_y) = y.checked_add_signed(-1) {
                            if guard_map[new_y][x] != b'#' as u8 && guard_map[new_y][x] != b'O' as u8{
                                guard_map[new_y][x] = '^' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                if turn_points.contains(&(x, y)) {
                                    return true;
                                }
                                turn_points.insert((x, y));
                                guard_map[y][x] = '>' as u8;
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == '>' {
                        if x + 1 < guard_map[y].len() {
                            if guard_map[y][x + 1] != b'#' as u8 && guard_map[y][x + 1] != b'O' as u8{
                                guard_map[y][x + 1] = '>' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                if turn_points.contains(&(x, y)) {
                                    return true;
                                }
                                turn_points.insert((x, y));
                                guard_map[y][x] = 'v' as u8;
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == 'v' {
                        if y + 1 < guard_map[y].len() {
                            if guard_map[y + 1][x] != b'#' as u8 && guard_map[y + 1][x] != b'O' as u8{
                                guard_map[y + 1][x] = 'v' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                if turn_points.contains(&(x, y)) {
                                    return true;
                                }
                                turn_points.insert((x, y));
                                guard_map[y][x] = '<' as u8;
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                    else if char::from(guard_map[y][x]) == '<' {
                        if let Some(new_x) = x.checked_add_signed(-1) {
                            if guard_map[y][new_x] != b'#' as u8 && guard_map[y][new_x] != b'O' as u8{
                                guard_map[y][new_x] = '<' as u8;
                                guard_map[y][x] = 'X' as u8;
                            }
                            else {
                                if turn_points.contains(&(x, y)) {
                                    return true;
                                }
                                turn_points.insert((x, y));
                                guard_map[y][x] = '^' as u8;
                            }
                        }
                        else {
                            guard_map[y][x] = 'X' as u8;
                            finished = true;
                        }
                    }
                //print!("{}", guard_map[y][x] as char);
            }
            //println!();
        }
    }
    false
}

fn print_map(guard_map: &Vec<Vec<u8>>) {
    for y in 0..guard_map.len() {
        for x in 0..guard_map[y].len() {
            print!("{}", guard_map[y][x] as char);
        }
        println!();
     }
}

pub fn run_day_6_part_2(contents: &str) -> impl Display + use<'_> {
    // convert to vec of byte array
    let guard_map = contents.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut loop_count = 0;
    for y in 0..guard_map.len() {
        for x in 0..guard_map[y].len() {
            if guard_map[y][x] != '#' as u8 && guard_map[y][x] != '^' as u8{
                println!("testing x: {}, y: {}", x, y);
                let mut map =guard_map.clone();
                map[y][x] = 'O' as u8;
                if loop_checker(&mut map) {
                    println!("looping!");
                    loop_count += 1;
                }
                else {
                    println!("no loop");
                }
                print_map(&map);
            }
        }
        //println!()
     }
    loop_count
}