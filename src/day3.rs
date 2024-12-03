const mul_func: &str = "mul(";

pub fn run_day_3_part_1(contents: String) {
    println!("Running day 3 part 1");
    for line in contents.lines() {
        println!("{}", line);
        let indices: Vec<(usize, &str)> = line.match_indices(mul_func).collect();
        for (index, _) in indices {
            println!("Index: {}", index);
        }
    }
}
