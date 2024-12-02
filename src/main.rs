use std::{collections::HashMap, env, fs};

fn run_day_1(contents: String) {
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

    // second part -> similarity score
    let mut similarity_score = 0;
    let mut right_number_map = HashMap::<i32, i32>::new();
    for i in 0..list2.len() {
        let number = list2[i];
        right_number_map
            .entry(number)
            .and_modify(|numb| *numb += 1)
            .or_insert(1);
    }

    for i in 0..list1.len() {
        let number = list1[i];
        if right_number_map.contains_key(&number) {
            similarity_score += number * right_number_map.get(&number).unwrap();
        }
    }

    println!("Similarity score: {similarity_score}");
}

fn safe_value_checker(array: Vec<i32>) -> bool {
    let mut previous = array[0];
    let mut current = array[1];
    let mut is_increasing = true;
    let mut is_unsafe = false;
    if previous > current {
        is_increasing = false;
    }
    for i in 1..array.len() {
        previous = array[i - 1];
        current = array[i];
        if previous == current {
            is_unsafe = true;
        } else if (previous - current).abs() > 3 {
            is_unsafe = true;
        } else if is_increasing && previous > current {
            is_unsafe = true;
        } else if !is_increasing && previous < current {
            is_unsafe = true;
        }
    }
    if !is_unsafe {
        println!("safe report");
        return true;
    }
    return false;
}

fn run_day_2(contents: String) {
    let lines = contents.lines();
    let mut number_of_safe_reports = 0;
    for line in lines {
        println!("{line}");
        let array = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if safe_value_checker(array.clone()) {
            number_of_safe_reports += 1;
        }
        else {
            for i in 0..array.len() {
                let mut new_array = array.clone();
                new_array.remove(i);
                if safe_value_checker(new_array) {
                    number_of_safe_reports += 1;
                    break;
                }
            }
        }
    }
    println!("Number of safe reports: {number_of_safe_reports}");
}

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
        "day1" => run_day_1(contents),
        "day2" => run_day_2(contents),
        _ => println!("Invalid query"),
    }
}
