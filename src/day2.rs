pub fn run_day_2_part_1(contents: String) {
    let lines = contents.lines();
    let mut number_of_safe_reports = 0;
    for line in lines {
        println!("{line}");
        let array = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
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
            number_of_safe_reports += 1;
        }
    }
    println!("Number of safe reports: {number_of_safe_reports}");
}

fn safe_value_checker(array: &Vec<i32>) -> bool {
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

pub fn run_day_2_part_2(contents: String) {
    let lines = contents.lines();
    let mut number_of_safe_reports = 0;
    for line in lines {
        println!("{line}");
        let array = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if safe_value_checker(&array) {
            number_of_safe_reports += 1;
        } else {
            for i in 0..array.len() {
                let mut new_array = array.clone();
                new_array.remove(i);
                if safe_value_checker(&new_array) {
                    number_of_safe_reports += 1;
                    break;
                }
            }
        }
    }
    println!("Number of safe reports: {number_of_safe_reports}");
}
