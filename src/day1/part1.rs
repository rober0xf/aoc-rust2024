use std::fs;

pub fn solve() {
    let file_path = "./src/day1/input.txt";
    let numbers = fs::read_to_string(file_path).expect("could not read the file");

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    // returns an iterator over the rows of the string
    for row in numbers.lines() {
        // parts will an iterator over the numbers of the line. separated with spaces. and cast the to i64
        let mut parts = row
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("invalid number"));

        // if there are two values, we assign them to left and right
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l);
            right.push(r);
        }
    }

    left.sort();
    right.sort();

    let mut diff: i64 = 0;
    for n in 0..left.len() {
        let current_diff: i64 = (right[n] - left[n]).abs();
        diff += current_diff;
    }

    println!("result: {}", diff);
}
