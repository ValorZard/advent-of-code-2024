use std::{error::Error, fmt::Display};

const MUL_FUNC_STARTER: &str = "mul(";

fn seek_parameters_and_multiply(remainder: &str) -> Result<i32, Box<dyn Error>> {
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
        } else if char == ')' {
            if param1.is_empty() || param2.is_empty() {
                return Err("Could not find both parameters".into());
            }
            return Ok(param1.parse::<i32>().unwrap() * param2.parse::<i32>().unwrap());
        } else if char == ',' {
            found_param1 = true;
            continue;
        } else {
            break;
        }
    }

    return Err("Could not find closing parenthesis".into());
}

pub fn run_day_3_part_1(contents: &str) -> impl Display {
    let mut total_value = 0;
    contents.split(MUL_FUNC_STARTER).for_each(|line| {
        if let Ok(value) = seek_parameters_and_multiply(line) {
            total_value += value;
        }
    });
    total_value
}

const DO_STARTER: &str = "do()";
const DONT_STARTER: &str = "don't()";

pub fn run_day_3_part_2(contents: &str) -> impl Display {
    //println!("Running day 3 part 2");
    let mut total_value = 0;
    //println!("Starting line: {}", contents);

    let lines = contents.split(DONT_STARTER).collect::<Vec<&str>>();

    let mut sorted_lines = Vec::<&str>::new();

    // push back first dont line since its good
    sorted_lines.push(lines[0]);

    // for the rest of the lines, we want to split at do() and push the remainder to sorted_lines
    for i in 1..lines.len() {
        let new_line = lines[i];
        let new_line_split = new_line.split(DO_STARTER).collect::<Vec<&str>>();
        // ignore the first element since it doesn't count (don't() ... do())
        // we don't care if the rest have doesn't have do() since we're not looking for it
        for j in 1..new_line_split.len() {
            sorted_lines.push(new_line_split[j]);
        }
    }

    for line in sorted_lines {
        let indices: Vec<(usize, &str)> = line.match_indices(MUL_FUNC_STARTER).collect();

        for (index, _) in indices {
            let remainder = &line[(index + MUL_FUNC_STARTER.len())..].to_string();
            //println!("Index: {}, String: {}", index, remainder);
            if let Ok(value) = seek_parameters_and_multiply(remainder) {
                //println!("Value: {}", value);
                total_value += value;
            }
        }
    }
    //println!("Total value: {}", total_value);
    total_value
}
