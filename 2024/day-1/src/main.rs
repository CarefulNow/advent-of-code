use std::fs;

fn main() {
    let locations_file = fs::read_to_string("./day-1/input.txt")
        .expect("Unable to read file");

    let mut locations_list_one: Vec<i64> = Vec::new();
    let mut locations_list_two: Vec<i64> = Vec::new();
    for line in locations_file.lines() {
        let location_line: Vec<&str> = line.split("   ").collect();
        locations_list_one.push(location_line[0].parse::<i64>().unwrap());
        locations_list_two.push(location_line[1].parse::<i64>().unwrap());
    }
    locations_list_one.sort();
    locations_list_two.sort();

    let length = locations_list_one.len();
    let mut total_distance = 0;
    let mut similarity_score = 0;
    for i in 0..length {
        total_distance += locations_list_one[i].abs_diff(locations_list_two[i]);
        similarity_score += locations_list_one[i] * locations_list_two
            .iter()
            .filter(|&n| *n == locations_list_one[i])
            .count() as i64;
    }

    println!("Total Distance: {}", total_distance);
    println!("Similarity Score: {}", similarity_score);
}
