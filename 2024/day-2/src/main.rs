use std::fs;

fn main() {
    let levels_file = fs::read_to_string("./day-2/input.txt")
        .expect("Unable to read file");

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in levels_file.lines() {
        let levels_line: Vec<i64> = line.split(" ")
            .map(|i| i.parse::<i64>())
            .map(Result::unwrap)
            .collect();

        reports.push(levels_line);
    }

    let mut num_safe_with_1_failure = 0;
    let mut num_safe_with_no_failure = 0;
    for report in reports {
        let mut increasing: bool = true;
        let mut prev_level: i64 = 0;
        let mut report_failures: i32 = 0;
        for (i, level) in report.iter().enumerate() {
            if i == 1 && *level < prev_level {
                increasing = false;
            }
            let is_after_first_element = i > 0;
            let diff_between_levels = level.abs_diff(prev_level);
            if is_after_first_element {
                if increasing && *level < prev_level {
                    report_failures += 1;
                }
                if !increasing && *level > prev_level {
                    report_failures += 1;
                }
                if diff_between_levels > 3 || diff_between_levels == 0 {
                    report_failures += 1;
                }
            }
            prev_level = *level
        }
        if report_failures <= 1 {
            num_safe_with_1_failure += 1
        }
        if report_failures == 0 {
            num_safe_with_no_failure += 1
        }
    }

    println!("Number of Safe Reports with No Failures: {}", num_safe_with_no_failure);
    println!("Number of Safe Reports with 1 Failure: {}", num_safe_with_1_failure);
}
