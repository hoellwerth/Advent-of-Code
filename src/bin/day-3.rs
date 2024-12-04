use regex::Regex;
use std::fs;

fn mulitply(input: String) {

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut enabled: bool = true;
    let mut total: i32 = 0;

    let segments = input.split("don't()").collect::<Vec<_>>();

    for (i, segment) in segments.iter().enumerate() {

        if i > 0 {
            enabled = false;
        }

        let sub_segments = segment.split("do()").collect::<Vec<_>>();

        for (j, sub_segment) in sub_segments.iter().enumerate() {

            if j > 0 {
                enabled = true;
            }

            if enabled {
                for cap in mul_re.captures_iter(sub_segment) {
                    let num1: i32 = cap[1].parse().unwrap();
                    let num2: i32 = cap[2].parse().unwrap();
                    total += num1 * num2;
                }
            }
        }
    }

    // Output the result
    println!("The total sum of valid mul instructions is: {}", total);
}


fn main() {
    let file_path: &str = "inputs/day-3.txt";

    let input: String = fs::read_to_string(file_path).expect("Failed to read file");
    mulitply(input);
}