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
        let mut dont_iterator = dont_indices.iter();
        let mut do_iterator = do_indices.iter();

        if let Some(dont_index) = dont_iterator.next() {
            if let Some(do_index) = do_iterator.next() {
                if dont_index.0 < do_index.0 {
                    do_pairs.push((dont_index.0, do_index.0));
                    
                    while let Some(dont_index) = dont_iterator.next() {
                        if let Some(do_index) = do_iterator.next() {
                            if dont_index.0 < do_index.0 {
                                do_pairs.push((dont_index.0, do_index.0));
                            }
                        }
                    }
                }
            }
        }

        for (dont_index, do_index) in &do_pairs {
            println!("Dont index: {}, Do: {}", dont_index, do_index);
        }


        let sorted_indices = indices.into_iter().filter(|(index, _)| {
            for (dont_index, do_index) in &do_pairs {
                if *index > *dont_index && *index < *do_index {
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
