use std::fs;
use regex::Regex;

fn main() {
    let mul_file: String = fs::read_to_string("./day-3/input.txt")
        .expect("Unable to read file");

    let mul_regex: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_regex: Regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut do_occurances: Vec<u64> = Vec::new();
    for do_match in do_regex.find_iter(&mul_file.to_string()) {
        do_occurances.push(do_match.start() as u64)
    }

    let mut dont_occurances: Vec<u64> = Vec::new();
    for dont_match in dont_regex.find_iter(&mul_file.to_string()) {
        dont_occurances.push(dont_match.start() as u64)
    }

    let mut total: i64 = 0;
    for capture in mul_regex.captures_iter(&mul_file.to_string()) {
        let first_cap_start: u64 = capture.get(0).unwrap().start() as u64;
        let mut nearest_do = nearest_value(first_cap_start, &do_occurances);
        let nearest_dont = nearest_value(first_cap_start, &dont_occurances);
        if nearest_do == 0 {
            nearest_do += 1
        }
        if first_cap_start.abs_diff(nearest_do) < first_cap_start.abs_diff(nearest_dont) {
            total += capture.get(1).unwrap().as_str().parse::<i64>().expect("Nan") * capture.get(2).unwrap().as_str().parse::<i64>().expect("Nan");
        }
    }

    println!("Total of all muls: {}", total);
}

fn nearest_value(value: u64, list: &Vec<u64>) -> u64 {
    let mut nearest_value: u64 = 0;
    for v in list {
        if *v > value {
            break
        }
        if v.abs_diff(value) < nearest_value.abs_diff(value) {
            nearest_value = *v;
        }
    }
    nearest_value
}
