use std::{env, fs};

mod day1;
mod day2;
mod day3;

fn main() {
    // run like so: cargo run -- test poem.txt
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query}");

    println!("In file {filename}");

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    //println!("With text:\n{contents}");

    match query.as_str() {
        "day1" => day1::run_day_1(contents),
        "day2-part1" => day2::run_day_2_part_1(contents),
        "day2-part2" => day2::run_day_2_part_2(contents),
        "day3-part1" => println!("Result: {}", day3::run_day_3_part_1(&contents)),
        "day3-part2" => println!("Result: {}", day3::run_day_3_part_2(&contents)),
        _ => println!("Invalid query"),
    }
}
