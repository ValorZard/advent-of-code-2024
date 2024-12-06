use std::fmt::Display;

pub fn run_day_6_part_1(contents: &str) -> impl Display + use<'_> {
    // convert to vec of byte array
    let mut guard_map = contents.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    for y in 0..guard_map.len() {
       for x in 0..guard_map[y].len() {
            if char::from(guard_map[y][x]) == '^' {
                if let Some(value) = y.checked_add_signed(-1) {
                    if guard_map[value][x] != b'#' as u8 {
                        guard_map[value][x] = '^' as u8;
                        guard_map[y][x] = 'X' as u8;
                    }
                }
            }
           print!("{}", guard_map[y][x] as char);
       }
       println!()
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