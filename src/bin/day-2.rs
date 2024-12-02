use std::fs;

fn is_safe_b(input: &[i32], skip: Option<usize>) -> bool {
    // Collect filtered values into a vector
    let mut vals = Vec::new();
    for (idx, &x) in input.iter().enumerate() {
        if skip.is_none() || Some(idx) != skip {
            vals.push(x);
        }
    }

    // Calculate differences between consecutive values
    let mut diffs = Vec::new();
    for i in 0..vals.len() - 1 {
        diffs.push(vals[i] - vals[i + 1]);
    }

    // Determine the sign of the first difference
    if diffs.is_empty() {
        return true; // No differences to evaluate
    }

    let sig = diffs[0].signum();

    // Find the first invalid difference
    let mut first_invalid = None;
    for (i, &diff) in diffs.iter().enumerate() {
        if !(1..=3).contains(&diff.abs()) || diff.signum() != sig {
            first_invalid = Some(i);
            break;
        }
    }

    // Evaluate the result based on the first invalid position
    match first_invalid {
        Some(x) if skip.is_none() => {
            is_safe_b(input, Some(x + 1))
                || is_safe_b(input, Some(x.saturating_sub(1)))
                || is_safe_b(input, Some(x))
        }
        None => true,
        _ => false,
    }
}

fn main() {
    
    let file_path: &str = "inputs/day-2.txt";

    let contents = fs::read_to_string(file_path).expect("");

    // Split the content into lines
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut safe_levels = 0;

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let numbers: Vec<i32> = words.iter().map(|w| w.parse::<i32>().unwrap()).collect();

        if is_safe_b(&numbers, None) {
            safe_levels += 1;
        }

        // Debugging
        println!("Line: {:?}, Safe: {}", numbers, is_safe_b(&numbers, None));
    }

    println!("Safe Levels: {}", safe_levels);
}
