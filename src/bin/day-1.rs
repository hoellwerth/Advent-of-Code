use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: &str = "inputs/day-1.txt";
    
    let contents = fs::read_to_string(file_path).expect("");

    // Split the content into lines
    let lines: Vec<&str> = contents.split('\n').collect();

    // split the lines into two arrays
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for element in lines {
        let words = element.split("   ").collect::<Vec<_>>();

        if words.len() == 2 {
    
            right.push(words[words.len() - 1].parse::<i32>().unwrap());
            left.push(words[0].parse::<i32>().unwrap());
        }
    }

    // Sort both arrays
    left.sort();
    right.sort();

    let mut distance: i32 = 0;
    let mut score: i32 = 0;
    let mut b_occurrences: HashMap<i32, i32> = HashMap::new();

    // Loop over both arrays
    for i in 0..left.len() {
        // Calculate distance between the two
        distance += (left[i] - right[i]).abs();
    }

    for &b in &right {
        let count = b_occurrences.entry(b).or_insert(0);
        *count += 1;
    }

    // Compute the total distance
    for &a in &left {
        if let Some(count) = b_occurrences.get(&a) {
            score += count * a;
        }
    }

    println!("Distance between both lists is: {}", distance);
    println!("Score between both lists is:    {}", score);
}
