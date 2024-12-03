use std::error::Error;

const MUL_FUNC_STARTER: &str = "mul(";

fn seek_parameters_and_multiply(remainder: &String) -> Result<i32, Box<dyn Error>>{
    let mut param1 = String::new();
    let mut param2 = String::new();

    let mut found_param1 = false;
    for char in remainder.chars() {
        if char.is_ascii_digit() {
            if !found_param1 {
                param1.push(char);
            } else {
                param2.push(char);
            }
        }
        else if char == ')' {
            if param1.is_empty() || param2.is_empty() {
                return Err("Could not find both parameters".into());
            }
            return Ok(param1.parse::<i32>().unwrap() * param2.parse::<i32>().unwrap());
        }
        else if char == ',' {
            found_param1 = true;
            continue;
        }
        else {
            break;
        }
    }

    return Err("Could not find closing parenthesis".into());
}

pub fn run_day_3_part_1(contents: String) {
    println!("Running day 3 part 1");
    let mut total_value = 0;
    for line in contents.lines() {
        println!("{}", line);
        let indices: Vec<(usize, &str)> = line.match_indices(MUL_FUNC_STARTER).collect();
        for (index, _) in indices {
            let remainder = &line[(index + MUL_FUNC_STARTER.len())..].to_string();
            println!("Index: {}, String: {}", index, remainder);
            if let Ok(value) = seek_parameters_and_multiply(remainder) {
                println!("Value: {}", value);
                total_value += value;
            } 
        }
    }
    println!("Total value: {}", total_value);
}

const DO_STARTER: &str = "do()";
const DONT_STARTER: &str = "don't()";

pub fn run_day_3_part_2(contents: String) {
    println!("Running day 3 part 1");
    let mut total_value = 0;
    for line in contents.lines() {
        println!("{}", line);
        let indices: Vec<(usize, &str)> = line.match_indices(MUL_FUNC_STARTER).collect();
        let do_indices: Vec<(usize, &str)> = line.match_indices(DO_STARTER).collect();
        let dont_indices: Vec<(usize, &str)> = line.match_indices(DONT_STARTER).collect();
        // each pair is a (DONT, DO) pair
        let mut do_pairs = Vec::<(usize, usize)>::new();
        let mut dont_iterator = dont_indices.into_iter();
        let mut do_iterator = do_indices.into_iter();
        let mut current_dont_index = dont_iterator.next().unwrap().0;
        let mut current_do_index = do_iterator.next().unwrap().0;
        while do_iterator.len() > 0 && dont_iterator.len() > 0 {
            if (current_dont_index > current_do_index) {
                do_pairs.push((current_do_index, current_dont_index));
                current_do_index = do_iterator.next().unwrap().0;
                current_dont_index = dont_iterator.next().unwrap().0;
            }
            else {
                current_dont_index = dont_iterator.next().unwrap().0;
            }
        }

        let sorted_indices = indices.into_iter().filter(|(index, _)| {
            for (do_start, dont) in &do_pairs {
                if *index > *do_start && *index < *dont {
                    return false;
                }
            }
            true
        }).collect::<Vec<_>>();

        for (index, _) in sorted_indices {
            let remainder = &line[(index + MUL_FUNC_STARTER.len())..].to_string();
            println!("Index: {}, String: {}", index, remainder);
            if let Ok(value) = seek_parameters_and_multiply(remainder) {
                println!("Value: {}", value);
                total_value += value;
            } 
        }
    }
    println!("Total value: {}", total_value);
}
