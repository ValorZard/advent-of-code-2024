use std::{collections::{HashMap, HashSet}, fmt::Display};

fn check_if_valid(pages: &Vec<usize>, num_pairs: &HashMap<usize, HashSet<usize>>) -> bool {
    println!("Valid checking: {:?}", pages);
    let mut list_of_pages_not_allowed_to_come_before: HashSet<usize> = HashSet::new();
    for page in pages.into_iter().rev() {
        if list_of_pages_not_allowed_to_come_before.contains(&page) {
            return false;
        }
        if let Some(forbidden_pages) = num_pairs.get(&page) {
            list_of_pages_not_allowed_to_come_before.extend(forbidden_pages);
        }
        println!("forbidden pages: {:?}", list_of_pages_not_allowed_to_come_before);
    }
    true
}
// thanks to ludi_61695 on the Rust Discord for the help with this one
pub fn run_day_5_part_1(contents: &str) -> impl Display + use<'_> {
    let mut num_pairs : HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut getting_pairs = true;
    let mut sum = 0;
    for line in contents.lines() {
            if getting_pairs {
                if line.is_empty() {
                    getting_pairs = false;
                    continue;
                }
                let pair = line.split("|").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                // pair[0] is the first page, pair[1] is the page that comes after
                num_pairs.entry(pair[0]).or_insert(HashSet::new()).insert(pair[1]);
                println!("{:?}", num_pairs);
            }
            else {
                let pages = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                if check_if_valid(&pages, &num_pairs){
                    println!("Valid");
                    // add middle page to sum
                    sum += pages[pages.len() / 2];
                }
            }
    }
    sum
}


pub fn run_day_5_part_2(contents: &str) -> impl Display + use<'_> {
    contents
}