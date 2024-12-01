use std::{env, fs};

fn main() {
    // run like so: cargo run -- test poem.txt
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query}");

    println!("In file {filename}");

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    let lines = contents.lines();

    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in lines {
        let data_pair = line.split_whitespace().collect::<Vec<&str>>();
        let first = data_pair[0];
        list1.push(first.parse::<i32>().unwrap());
        let second = data_pair[1];
        list2.push(second.parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut total_distance = 0;
    for i in 0..list1.len() {
        total_distance += (list1[i] - list2[i]).abs();
    }
    println!("Total distance: {total_distance}");
}
